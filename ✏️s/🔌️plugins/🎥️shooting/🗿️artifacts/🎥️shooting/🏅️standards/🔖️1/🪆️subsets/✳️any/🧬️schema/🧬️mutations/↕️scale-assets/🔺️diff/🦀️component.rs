//! 🔺 Diff constructor for `ScaleAssets` — multiplies each targeted asset's current per-axis scale
//! (identity when absent) by the payload factors.

use super::mutation::ScaleAssets;
use crate::artifacts::shooting::diff::{ShootingAssetPatchEntry, ShootingAssetsDelta, ShootingDiff};
use crate::artifacts::shooting::{shooting_asset_scale, ShootingAssetPatch, ShootingSnapshot};

//#region ↕️ScaleAssets
pub fn diff_scale_assets(payload: &ScaleAssets, base: &ShootingSnapshot) -> ShootingDiff {
    let patched: Vec<ShootingAssetPatchEntry> = base
        .assets
        .iter()
        .filter(|asset| payload.asset_ids.contains(&asset.id))
        .map(|asset| {
            let current = shooting_asset_scale(asset);
            ShootingAssetPatchEntry { id: asset.id.clone(), patch: ShootingAssetPatch { scale: Some([current[0] * payload.sx, current[1] * payload.sy, current[2] * payload.sz]), ..Default::default() } }
        })
        .collect();
    if patched.is_empty() {
        return ShootingDiff::default();
    }
    ShootingDiff { assets: Some(ShootingAssetsDelta { patched, ..Default::default() }), ..Default::default() }
}
//#endregion ↕️ScaleAssets
