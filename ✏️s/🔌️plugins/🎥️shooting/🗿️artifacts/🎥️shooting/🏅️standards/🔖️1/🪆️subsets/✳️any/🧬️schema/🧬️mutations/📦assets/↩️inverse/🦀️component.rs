//! ↩ Inverse constructors for the `assets` collection's mutation kinds — reconstructed from
//! captured BASE state. Missing target ⇒ `Vec::new()`.

use super::mutation::{ChangeAssetUrl, CreateAsset, DeleteAsset, RenameAsset, ReorderAssets};
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;

//#region 🌱️CreateAsset
pub fn inverse_create_asset(payload: &CreateAsset, _base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::DeleteAsset(DeleteAsset { id: payload.asset.id.clone() })]
}
//#endregion 🌱️CreateAsset

//#region 🗑️DeleteAsset
pub fn inverse_delete_asset(payload: &DeleteAsset, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.assets.iter().position(|asset| asset.id == payload.id) {
        Some(index) => vec![ShootingMutation::CreateAsset(CreateAsset { asset: base.assets[index].clone(), index: Some(index) })],
        None => Vec::new(),
    }
}
//#endregion 🗑️DeleteAsset

//#region ✏️RenameAsset
pub fn inverse_rename_asset(payload: &RenameAsset, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.assets.iter().find(|asset| asset.id == payload.id) {
        Some(asset) => vec![ShootingMutation::RenameAsset(RenameAsset { id: payload.id.clone(), new_name: asset.name.clone() })],
        None => Vec::new(),
    }
}
//#endregion ✏️RenameAsset

//#region 🔗️ChangeAssetUrl
pub fn inverse_change_asset_url(payload: &ChangeAssetUrl, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.assets.iter().find(|asset| asset.id == payload.id) {
        Some(asset) => vec![ShootingMutation::ChangeAssetUrl(ChangeAssetUrl { id: payload.id.clone(), new_url: asset.url.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔗️ChangeAssetUrl

//#region 🔀️ReorderAssets
pub fn inverse_reorder_assets(payload: &ReorderAssets, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.assets.iter().position(|asset| asset.id == payload.id) {
        Some(original_index) => vec![ShootingMutation::ReorderAssets(ReorderAssets { id: payload.id.clone(), to_index: original_index })],
        None => Vec::new(),
    }
}
//#endregion 🔀️ReorderAssets
