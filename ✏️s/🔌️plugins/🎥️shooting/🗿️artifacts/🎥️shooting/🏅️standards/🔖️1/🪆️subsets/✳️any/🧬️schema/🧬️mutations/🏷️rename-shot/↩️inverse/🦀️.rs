//! ↩ Inverse constructor for `RenameShot` — reconstructed from BASE state.

use super::RenameShot;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(payload: &RenameShot, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.shots.iter().find(|shot| shot.id == payload.id) {
        Some(shot) => vec![ShootingMutation::RenameShot(RenameShot { id: payload.id.clone(), new_label: shot.label.clone() })],
        None => Vec::new(),
    }
}
