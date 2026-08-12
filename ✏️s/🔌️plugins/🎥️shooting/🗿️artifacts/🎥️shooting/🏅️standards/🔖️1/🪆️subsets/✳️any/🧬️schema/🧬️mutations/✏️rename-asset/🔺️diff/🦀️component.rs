//! 🔺 Diff constructor for `RenameAsset`.

use super::mutation::RenameAsset;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingAssetPatchEntry, ShootingAssetsDelta, ShootingDiff};
use crate::artifacts::shooting::ShootingAssetPatch;

pub fn diff(payload: &RenameAsset, base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff {
        assets: Some(ShootingAssetsDelta {
            patched: vec![ShootingAssetPatchEntry { id: payload.id.clone(), patch: ShootingAssetPatch { name: Some(payload.new_name.clone()), ..Default::default() } }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
