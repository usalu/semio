//! 🔺 Diff constructor for `RotateAssets`.

use super::mutation::RotateAssets;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingAssetPatchEntry, ShootingAssetsDelta, ShootingDiff};
use crate::artifacts::shooting::{quat_from_axis_angle, quat_mul, ShootingAssetPatch};

pub fn diff(payload: &RotateAssets, base: &ShootingSnapshot) -> ShootingDiff {
    let delta = quat_from_axis_angle(payload.ax, payload.ay, payload.az, payload.angle);
    let patched: Vec<ShootingAssetPatchEntry> = base
        .assets
        .iter()
        .filter(|asset| payload.asset_ids.contains(&asset.id))
        .map(|asset| {
            let current = asset.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
            ShootingAssetPatchEntry { id: asset.id.clone(), patch: ShootingAssetPatch { orientation: Some(quat_mul(delta, current)), ..Default::default() } }
        })
        .collect();
    if patched.is_empty() {
        return ShootingDiff::default();
    }
    ShootingDiff { assets: Some(ShootingAssetsDelta { patched, ..Default::default() }), ..Default::default() }
}
