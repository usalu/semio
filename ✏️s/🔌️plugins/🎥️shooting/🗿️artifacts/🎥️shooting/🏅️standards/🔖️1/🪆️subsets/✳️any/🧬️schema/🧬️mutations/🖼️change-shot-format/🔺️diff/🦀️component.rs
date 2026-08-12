//! 🔺 Diff constructor for `ChangeShotFormat`.

use super::mutation::ChangeShotFormat;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingShotPatchEntry, ShootingShotsDelta, ShootingDiff};
use crate::artifacts::shooting::ShootingShotPatch;

pub fn diff(payload: &ChangeShotFormat, base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff {
        shots: Some(ShootingShotsDelta { patched: vec![ShootingShotPatchEntry { id: payload.id.clone(), patch: ShootingShotPatch { format: Some(payload.new_format.clone()), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    }
}
