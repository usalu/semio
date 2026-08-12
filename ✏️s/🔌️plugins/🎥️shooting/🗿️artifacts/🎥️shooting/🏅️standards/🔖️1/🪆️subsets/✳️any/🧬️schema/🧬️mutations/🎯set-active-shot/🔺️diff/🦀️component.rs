//! 🔺 Diff constructor for `SetActiveShot`.

use super::mutation::SetActiveShot;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::ShootingDiff;

pub fn diff(payload: &SetActiveShot, base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff { active_shot_id: Some(payload.shot_id.clone().unwrap_or_default()), ..Default::default() }
}
