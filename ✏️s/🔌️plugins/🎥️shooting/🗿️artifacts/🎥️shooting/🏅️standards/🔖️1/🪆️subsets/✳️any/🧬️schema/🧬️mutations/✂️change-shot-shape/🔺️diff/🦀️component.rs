//! 🔺 Diff constructor for `ChangeShotShape`. Error `target-missing` when absent, Warning `no-op`
//! when already at that shape.

use super::mutation::ChangeShotShape;
use crate::artifacts::shooting::diff::{ShootingDiff, ShootingShotPatchEntry, ShootingShotsDelta};
use crate::artifacts::shooting::ShootingShotPatch;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn diff(payload: &ChangeShotShape, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    let Some(existing) = base.shots.iter().find(|shot| shot.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Shot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.shape == payload.new_shape {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Shot \"{}\" already has shape \"{}\".", payload.id, payload.new_shape));
    }
    protocol::MutationOutcome::new(ShootingDiff {
        shots: Some(ShootingShotsDelta { patched: vec![ShootingShotPatchEntry { id: payload.id.clone(), patch: ShootingShotPatch { shape: Some(payload.new_shape.clone()), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
