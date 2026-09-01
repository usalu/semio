//! 🔺 Diff constructor for `SetActiveShot`. Error `target-missing` when addressing an unknown
//! shot, Warning `no-op` when already active.

use super::SetActiveShot;
use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn diff(payload: &SetActiveShot, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    let next = payload.shot_id.clone().unwrap_or_default();
    if let Some(id) = &payload.shot_id {
        if !base.shots.iter().any(|shot| &shot.id == id) {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("Shot \"{}\" does not exist.", id), [id.clone()]);
        }
    }
    if base.active_shot_id == next {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Active shot is unchanged.");
    }
    protocol::MutationOutcome::new(ShootingDiff { active_shot_id: Some(next), ..Default::default() })
}
