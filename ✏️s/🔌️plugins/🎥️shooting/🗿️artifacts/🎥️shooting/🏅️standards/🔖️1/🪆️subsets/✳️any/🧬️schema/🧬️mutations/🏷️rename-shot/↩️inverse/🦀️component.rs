//! ↩ Inverse constructor for `RenameShot` — reconstructed from BASE state.

use super::mutation::RenameShot;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;


pub fn inverse(payload: &RenameShot, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.shots.iter().find(|shot| shot.id == payload.id) {
        Some(shot) => vec![ShootingMutation::RenameShot(RenameShot { id: payload.id.clone(), new_label: shot.label.clone() })],
        None => Vec::new(),
    }
}
