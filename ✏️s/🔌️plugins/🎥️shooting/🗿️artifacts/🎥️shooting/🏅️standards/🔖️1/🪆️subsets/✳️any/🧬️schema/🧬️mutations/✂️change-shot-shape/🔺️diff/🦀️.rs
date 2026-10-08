//! 🔺 Diff constructor for `ChangeShotShape`. Error `target-missing` when absent, Warning `no-op`
//! when already at that shape.

use super::ChangeShotShape;
use crate::diff::ShootingDiff;
use crate::ShootingShotPatch;
use crate::ShootingSnapshot;

pub fn diff(payload: &ChangeShotShape, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    let Some(existing) = base.shots.iter().find(|shot| shot.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Shot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.shape == payload.new_shape {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Shot \"{}\" already has shape \"{}\".", payload.id, payload.new_shape));
    }
    protocol::MutationOutcome::new(ShootingDiff::shot_patches([(payload.id.clone(), ShootingShotPatch { shape: Some(payload.new_shape.clone()), ..Default::default() })]))
}
