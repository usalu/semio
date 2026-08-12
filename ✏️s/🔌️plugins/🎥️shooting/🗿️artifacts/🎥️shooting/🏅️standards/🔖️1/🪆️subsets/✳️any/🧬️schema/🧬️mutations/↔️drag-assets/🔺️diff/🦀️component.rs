//! 🔺 Diff constructor for `DragAssets`.

use super::mutation::DragAssets;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingAssetPatchEntry, ShootingAssetsDelta, ShootingDiff};
use crate::artifacts::shooting::ShootingAssetPatch;

pub fn diff(payload: &DragAssets, base: &ShootingSnapshot) -> ShootingDiff {
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
