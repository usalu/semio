//! ↩ Inverse constructor for `DeleteShot` — reconstructed from BASE state.

use super::DeleteShot;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(payload: &DeleteShot, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.shots.iter().position(|shot| shot.id == payload.id) {
        Some(index) => vec![ShootingMutation::CreateShot(crate::mutations::create_shot::CreateShot { shot: base.shots[index].clone(), index: Some(index) })],
        None => Vec::new(),
    }
}
