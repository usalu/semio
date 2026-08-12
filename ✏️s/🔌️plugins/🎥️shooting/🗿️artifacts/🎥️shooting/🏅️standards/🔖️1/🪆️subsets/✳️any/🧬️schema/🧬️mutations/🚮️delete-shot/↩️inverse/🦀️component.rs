//! ↩ Inverse constructor for `DeleteShot` — reconstructed from BASE state.

use super::mutation::DeleteShot;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;


pub fn inverse(payload: &DeleteShot, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.shots.iter().position(|shot| shot.id == payload.id) {
        Some(index) => vec![ShootingMutation::CreateShot(crate::artifacts::shooting::mutations::create_shot::mutation::CreateShot { shot: base.shots[index].clone(), index: Some(index) })],
        None => Vec::new(),
    }
}
