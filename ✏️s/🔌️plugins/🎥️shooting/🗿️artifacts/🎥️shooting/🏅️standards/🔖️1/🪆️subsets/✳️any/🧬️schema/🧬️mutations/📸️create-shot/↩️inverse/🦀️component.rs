//! ↩ Inverse constructor for `CreateShot` — reconstructed from BASE state.

use super::mutation::CreateShot;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn inverse(payload: &CreateShot, _base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::DeleteShot(crate::artifacts::shooting::mutations::delete_shot::mutation::DeleteShot { id: payload.shot.id.clone() })]
}
