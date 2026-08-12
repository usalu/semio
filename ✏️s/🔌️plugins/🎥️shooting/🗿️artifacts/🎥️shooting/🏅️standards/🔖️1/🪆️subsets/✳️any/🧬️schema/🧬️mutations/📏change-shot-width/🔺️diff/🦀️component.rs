//! 🔺 Diff constructor for `ChangeShotWidth`.

use super::mutation::ChangeShotWidth;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingShotPatchEntry, ShootingShotsDelta, ShootingDiff};
use crate::artifacts::shooting::ShootingShotPatch;

pub fn diff(payload: &ChangeShotWidth, base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff {
        shots: Some(ShootingShotsDelta { patched: vec![ShootingShotPatchEntry { id: payload.id.clone(), patch: ShootingShotPatch { width: Some(payload.new_width), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    }
}
