//! ↩ Inverse constructor for `SetActiveShot` — always applicable (the document root's
//! `active_shot_id` field always exists, no missing-target case).

use super::mutation::SetActiveShot;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;

//#region 🎯️SetActiveShot
pub fn inverse_set_active_shot(_payload: &SetActiveShot, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    let shot_id = if base.active_shot_id.is_empty() { None } else { Some(base.active_shot_id.clone()) };
    vec![ShootingMutation::SetActiveShot(SetActiveShot { shot_id })]
}
//#endregion 🎯️SetActiveShot
