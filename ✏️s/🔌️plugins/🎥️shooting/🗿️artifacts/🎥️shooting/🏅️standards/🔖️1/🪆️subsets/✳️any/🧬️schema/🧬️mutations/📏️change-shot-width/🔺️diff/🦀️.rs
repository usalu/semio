//! 🔺 Diff constructor for `ChangeShotWidth`. Error `target-missing` when absent, Warning `no-op`
//! when already at that width, Fatal `invariant` when the width is zero.

use super::ChangeShotWidth;
use crate::artifacts::shooting::diff::{ShootingDiff, ShootingShotPatchEntry, ShootingShotsDelta};
use crate::artifacts::shooting::ShootingShotPatch;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn diff(payload: &ChangeShotWidth, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    let Some(existing) = base.shots.iter().find(|shot| shot.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Shot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.new_width == 0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Shot \"{}\" width must be positive.", payload.id), [payload.id.clone()]);
    }
    if existing.width == payload.new_width {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Shot \"{}\" already has width {}.", payload.id, payload.new_width));
    }
    protocol::MutationOutcome::new(ShootingDiff {
        shots: Some(ShootingShotsDelta { patched: vec![ShootingShotPatchEntry { id: payload.id.clone(), patch: ShootingShotPatch { width: Some(payload.new_width), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
