//! 🔺 Diff constructor for `ChangeAssetUrl`.

use super::mutation::ChangeAssetUrl;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingAssetPatchEntry, ShootingAssetsDelta, ShootingDiff};
use crate::artifacts::shooting::ShootingAssetPatch;

pub fn diff(payload: &ChangeAssetUrl, base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff {
        assets: Some(ShootingAssetsDelta {
            patched: vec![ShootingAssetPatchEntry { id: payload.id.clone(), patch: ShootingAssetPatch { url: Some(payload.new_url.clone()), ..Default::default() } }],
            ..Default::default()
        }),
        ..Default::default()
    }
}
