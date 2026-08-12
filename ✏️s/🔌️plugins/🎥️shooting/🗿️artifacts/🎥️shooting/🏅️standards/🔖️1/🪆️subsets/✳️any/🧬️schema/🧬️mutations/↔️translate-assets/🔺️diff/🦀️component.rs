//! 🔺 Diff constructor for `DragAssets` — patches every targeted asset's `origin` by the offset.

use super::mutation::DragAssets;
use crate::artifacts::shooting::diff::{ShootingAssetPatchEntry, ShootingAssetsDelta, ShootingDiff};
use crate::artifacts::shooting::{ShootingAssetPatch, ShootingSnapshot};

//#region ↔️DragAssets
pub fn diff_drag_assets(payload: &DragAssets, base: &ShootingSnapshot) -> ShootingDiff {
    let patched: Vec<ShootingAssetPatchEntry> = base
        .assets
        .iter()
        .filter(|asset| payload.asset_ids.contains(&asset.id))
        .map(|asset| ShootingAssetPatchEntry {
            id: asset.id.clone(),
            patch: ShootingAssetPatch { origin: Some([asset.origin[0] + payload.dx, asset.origin[1] + payload.dy, asset.origin[2] + payload.dz]), ..Default::default() },
        })
        .collect();
    if patched.is_empty() {
        return ShootingDiff::default();
    }
    ShootingDiff { assets: Some(ShootingAssetsDelta { patched, ..Default::default() }), ..Default::default() }
}
//#endregion ↔️DragAssets
