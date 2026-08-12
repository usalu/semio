//! 🔺 Diff constructor for `RenameShot`.

use super::mutation::RenameShot;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingShotPatchEntry, ShootingShotsDelta, ShootingDiff};
use crate::artifacts::shooting::ShootingShotPatch;

pub fn diff(payload: &RenameShot, base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff {
        shots: Some(ShootingShotsDelta { patched: vec![ShootingShotPatchEntry { id: payload.id.clone(), patch: ShootingShotPatch { label: Some(payload.new_label.clone()), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    }
}
