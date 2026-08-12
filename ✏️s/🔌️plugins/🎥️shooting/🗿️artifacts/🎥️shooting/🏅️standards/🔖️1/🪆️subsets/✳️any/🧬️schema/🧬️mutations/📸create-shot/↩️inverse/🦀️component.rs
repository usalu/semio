//! ↩ Inverse constructor for `CreateShot` — reconstructed from BASE state.

use super::mutation::CreateShot;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;


pub fn inverse(payload: &CreateShot, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::DeleteShot(crate::artifacts::shooting::mutations::delete_shot::mutation::DeleteShot { id: payload.shot.id.clone() })]
}
