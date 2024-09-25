#![allow(non_snake_case)]
#![no_std]

use soroban_sdk::{contract, contracttype, contractimpl, Env, String, symbol_short, Vec};

// Struct for tracking licensing information
#[contracttype]
#[derive(Clone)]
pub struct License {
    pub licensee: String,       // Licensee address or ID
    pub usage_rights: String,   // Rights granted (e.g., reproduction, distribution)
    pub expiration_time: u64,   // Expiration of the license
    pub royalty_percentage: u32, // Percentage of royalty for each use
    pub is_active: bool,        // License status
}

// Struct for tracking digital content metadata
#[contracttype]
#[derive(Clone)]
pub struct DigitalAsset {
    pub asset_id: u64,          // Unique identifier for the digital asset
    pub title: String,          // Title of the digital asset
    pub description: String,    // Description of the digital asset
    pub owner: String,          // Owner of the digital asset
}

// Digital Rights Management Contract
#[contract]
pub struct DigitalRightsContract;

#[contractimpl]
impl DigitalRightsContract {
    // Function to create a new digital asset (content) with title, description, and owner
    pub fn create_asset(env: Env, title: String, description: String, owner: String) -> u64 {
        let asset_count_symbol = symbol_short!("AST_CNT");
        
        let mut asset_count: u64 = env.storage().instance().get(&asset_count_symbol).unwrap_or(0);
        asset_count += 1;

        let asset = DigitalAsset {
            asset_id: asset_count,
            title: title.clone(),
            description: description.clone(),
            owner: owner.clone(),
        };

        // Store the new asset data
        env.storage().instance().set(&Assetbook::Asset(asset.asset_id.clone()), &asset);
        env.storage().instance().set(&asset_count_symbol, &asset_count);

        asset.asset_id
    }

    // Function to create a license for a digital asset
    pub fn create_license(
        env: Env,
        asset_id: u64,
        licensee: String,
        usage_rights: String,
        royalty_percentage: u32,
        expiration_time: u64,
    ) {
        let license = License {
            licensee: licensee.clone(),
            usage_rights: usage_rights.clone(),
            royalty_percentage: royalty_percentage,
            expiration_time: expiration_time,
            is_active: true,
        };

        // Store the new license data for the given asset
        env.storage().instance().set(&Licensebook::License(asset_id.clone()), &license);
    }

    // Function to approve a license (allows the licensee to use the asset)
    pub fn approve_license(env: Env, asset_id: u64) {
        let mut license = Self::view_license(env.clone(), asset_id.clone());

        if !license.is_active {
            license.is_active = true;
            env.storage().instance().set(&Licensebook::License(asset_id.clone()), &license);
        } else {
            panic!("License already active or approved");
        }
    }

    // Function to expire a license
    pub fn expire_license(env: Env, asset_id: u64) {
        let mut license = Self::view_license(env.clone(), asset_id.clone());

        if license.is_active {
            license.is_active = false;
            env.storage().instance().set(&Licensebook::License(asset_id.clone()), &license);
        } else {
            panic!("License is already expired");
        }
    }

    // Function to view an asset's metadata by asset ID
    pub fn view_asset(env: Env, asset_id: u64) -> DigitalAsset {
        let key = Assetbook::Asset(asset_id.clone());

        env.storage().instance().get(&key).unwrap_or(DigitalAsset {
            asset_id: 0,
            title: String::from_str(&env, "Not Found"),
            description: String::from_str(&env, "Not Found"),
            owner: String::from_str(&env, "Unknown"),
        })
    }

    // Function to view the license details of an asset by asset ID
    pub fn view_license(env: Env, asset_id: u64) -> License {
        let key = Licensebook::License(asset_id.clone());

        env.storage().instance().get(&key).unwrap_or(License {
            licensee: String::from_str(&env, "Unknown"),
            usage_rights: String::from_str(&env, "Not Found"),
            expiration_time: 0,
            royalty_percentage: 0,
            is_active: false,
        })
    }

    // Function to view all assets of a given owner
    pub fn view_assets_by_owner(env: Env, owner: String) -> Vec<DigitalAsset> {
        let mut all_assets = Vec::new(&env);

        for i in 0..Self::asset_count(env.clone()) {
            let asset = Self::view_asset(env.clone(), i);
            if asset.owner == owner {
                all_assets.push_back(asset);
            }
        }

        all_assets
    }

    // Helper function to get the total count of assets
    pub fn asset_count(env: Env) -> u64 {
        let asset_count_symbol = symbol_short!("AST_CNT");
        env.storage().instance().get(&asset_count_symbol).unwrap_or(0)
    }
}

// Mapping DigitalAsset to asset ID
#[contracttype]
pub enum Assetbook {
    Asset(u64),
}

// Mapping License to asset ID
#[contracttype]
pub enum Licensebook {
    License(u64),
}
