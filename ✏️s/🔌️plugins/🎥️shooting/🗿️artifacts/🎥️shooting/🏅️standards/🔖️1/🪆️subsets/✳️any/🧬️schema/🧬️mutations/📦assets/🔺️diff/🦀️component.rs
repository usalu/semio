//! 🔺 Diff constructors for the `assets` collection's mutation kinds — each builds
//! [`ShootingDiff`] sparsely and directly from its payload, never apply-then-capture.

use super::mutation::{ChangeAssetUrl, CreateAsset, DeleteAsset, RenameAsset, ReorderAssets};
use crate::artifacts::shooting::diff::{ShootingAssetPatchEntry, ShootingAssetsDelta, ShootingDiff};
use crate::artifacts::shooting::{ShootingAssetPatch, ShootingSnapshot};

//#region 🌱️CreateAsset
pub fn diff_create_asset(payload: &CreateAsset, _base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff { assets: Some(ShootingAssetsDelta { added: vec![payload.asset.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🌱️CreateAsset

//#region 🗑️DeleteAsset
pub fn diff_delete_asset(payload: &DeleteAsset, _base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff { assets: Some(ShootingAssetsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
//#endregion 🗑️DeleteAsset

//#region ✏️RenameAsset
pub fn diff_rename_asset(payload: &RenameAsset, _base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff {
        assets: Some(ShootingAssetsDelta {
            patched: vec![ShootingAssetPatchEntry { id: payload.id.clone(), patch: ShootingAssetPatch { name: Some(payload.new_name.clone()), ..Default::default() } }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
//#endregion ✏️RenameAsset

//#region 🔗️ChangeAssetUrl
pub fn diff_change_asset_url(payload: &ChangeAssetUrl, _base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff {
        assets: Some(ShootingAssetsDelta {
            patched: vec![ShootingAssetPatchEntry { id: payload.id.clone(), patch: ShootingAssetPatch { url: Some(payload.new_url.clone()), ..Default::default() } }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
//#endregion 🔗️ChangeAssetUrl

//#region 🔀️ReorderAssets
pub fn diff_reorder_assets(payload: &ReorderAssets, base: &ShootingSnapshot) -> ShootingDiff {
    let mut ids: Vec<String> = base.assets.iter().map(|asset| asset.id.clone()).collect();
    if let Some(from) = ids.iter().position(|id| id == &payload.id) {
        let item = ids.remove(from);
        let to = payload.to_index.min(ids.len());
        ids.insert(to, item);
    }
    ShootingDiff { assets: Some(ShootingAssetsDelta { reordered: Some(ids), ..Default::default() }), ..Default::default() }
}
//#endregion 🔀️ReorderAssets
