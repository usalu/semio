//! 🔺 Diff constructor for `ChangeShotHeight`.

use super::mutation::ChangeShotHeight;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingShotPatchEntry, ShootingShotsDelta, ShootingDiff};
use crate::artifacts::shooting::ShootingShotPatch;

pub fn diff(payload: &ChangeShotHeight, base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff {
        shots: Some(ShootingShotsDelta { patched: vec![ShootingShotPatchEntry { id: payload.id.clone(), patch: ShootingShotPatch { height: Some(payload.new_height), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    }
}
