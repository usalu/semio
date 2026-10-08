//! ↩ Inverse constructor for `DeleteShot` — reconstructed from BASE state.

use super::DeleteShot;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(payload: &DeleteShot, base: &ShootingSnapshot) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    Ok(match base.shots.iter().position(|entry| entry.id == payload.id) {
        Some(index) => vec![ShootingMutation::CreateShot(crate::mutations::create_shot::CreateShot { shot: base.shots[index].clone(), index: Some(index) })],
        None => Vec::new(),
    })
}
