//! 🔺 Diff constructor for `ChangeShotHeight`. Error `target-missing` when absent, Warning `no-op`
//! when already at that height, Fatal `invariant` when the height is zero.

use super::ChangeShotHeight;
use crate::artifacts::shooting::diff::{ShootingDiff, ShootingShotPatchEntry, ShootingShotsDelta};
use crate::artifacts::shooting::ShootingShotPatch;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn diff(payload: &ChangeShotHeight, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    let Some(existing) = base.shots.iter().find(|shot| shot.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Shot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.new_height == 0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Shot \"{}\" height must be positive.", payload.id), [payload.id.clone()]);
    }
    if existing.height == payload.new_height {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Shot \"{}\" already has height {}.", payload.id, payload.new_height));
    }
    protocol::MutationOutcome::new(ShootingDiff {
        shots: Some(ShootingShotsDelta { patched: vec![ShootingShotPatchEntry { id: payload.id.clone(), patch: ShootingShotPatch { height: Some(payload.new_height), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
