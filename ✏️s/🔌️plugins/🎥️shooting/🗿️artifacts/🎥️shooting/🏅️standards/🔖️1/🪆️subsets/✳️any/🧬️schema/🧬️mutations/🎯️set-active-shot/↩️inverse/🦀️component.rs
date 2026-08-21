//! ↩ Inverse constructor for `SetActiveShot` — reconstructed from BASE state.

use super::mutation::SetActiveShot;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn inverse(_payload: &SetActiveShot, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    let shot_id = if base.active_shot_id.is_empty() { None } else { Some(base.active_shot_id.clone()) };
    vec![ShootingMutation::SetActiveShot(SetActiveShot { shot_id })]
}
