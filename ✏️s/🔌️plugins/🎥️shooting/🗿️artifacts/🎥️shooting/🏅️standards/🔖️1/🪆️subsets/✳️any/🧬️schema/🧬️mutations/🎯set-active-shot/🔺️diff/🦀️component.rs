//! 🔺 Diff constructor for `SetActiveShot`. `ShootingSnapshot::active_shot_id` is a required
//! (non-`Option`) `String` field, empty meaning "none" — `None` encodes to the empty string.

use super::mutation::SetActiveShot;
use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::ShootingSnapshot;

//#region 🎯️SetActiveShot
pub fn diff_set_active_shot(payload: &SetActiveShot, _base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff { active_shot_id: Some(payload.shot_id.clone().unwrap_or_default()), ..Default::default() }
}
//#endregion 🎯️SetActiveShot
