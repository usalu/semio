//! 🔺 Diff constructor for `ChangeShotFormat`. Error `target-missing` when absent, Warning `no-op`
//! when already at that format.

use super::mutation::ChangeShotFormat;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingShotPatchEntry, ShootingShotsDelta, ShootingDiff};
use crate::artifacts::shooting::ShootingShotPatch;

pub async fn diff(payload: &ChangeShotFormat, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    let Some(existing) = base.shots.iter().find(|shot| shot.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Shot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.format == payload.new_format {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Shot \"{}\" already has format \"{}\".", payload.id, payload.new_format));
    }
    protocol::MutationOutcome::new(ShootingDiff {
        shots: Some(ShootingShotsDelta { patched: vec![ShootingShotPatchEntry { id: payload.id.clone(), patch: ShootingShotPatch { format: Some(payload.new_format.clone()), ..Default::default() } }], ..Default::default() }),
        ..Default::default()
    })
}
