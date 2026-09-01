//! 🔺 Diff constructor for `RenameShot`. Error `target-missing` when absent, Warning `no-op` when
//! already at that label.

use super::RenameShot;
use crate::artifacts::shooting::diff::{ShootingDiff, ShootingShotPatchEntry, ShootingShotsDelta};
use crate::artifacts::shooting::ShootingShotPatch;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn diff(payload: &RenameShot, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    let Some(existing) = base.shots.iter().find(|shot| shot.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Shot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.label == payload.new_label {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Shot \"{}\" already has label \"{}\".", payload.id, payload.new_label));
    }
    protocol::MutationOutcome::new(ShootingDiff {
        shots: Some(ShootingShotsDelta { patched: vec![ShootingShotPatchEntry { id: payload.id.clone(), patch: ShootingShotPatch { label: Some(payload.new_label.clone()), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
