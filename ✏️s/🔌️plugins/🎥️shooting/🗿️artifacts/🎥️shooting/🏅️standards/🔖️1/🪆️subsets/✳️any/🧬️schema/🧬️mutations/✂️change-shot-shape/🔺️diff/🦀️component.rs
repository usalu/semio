//! 🔺 Diff constructor for `ChangeShotShape`.

use super::mutation::ChangeShotShape;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingShotPatchEntry, ShootingShotsDelta, ShootingDiff};
use crate::artifacts::shooting::ShootingShotPatch;

pub fn diff(payload: &ChangeShotShape, base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff {
        shots: Some(ShootingShotsDelta { patched: vec![ShootingShotPatchEntry { id: payload.id.clone(), patch: ShootingShotPatch { shape: Some(payload.new_shape.clone()), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    }
}
