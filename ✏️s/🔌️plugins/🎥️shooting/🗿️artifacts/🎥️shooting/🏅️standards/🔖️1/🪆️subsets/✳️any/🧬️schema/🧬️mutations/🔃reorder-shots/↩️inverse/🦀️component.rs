//! ↩ Inverse constructor for `ReorderShots` — reconstructed from BASE state.

use super::mutation::ReorderShots;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;


pub fn inverse(payload: &ReorderShots, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.shots.iter().position(|shot| shot.id == payload.id) {
        Some(original_index) => vec![ShootingMutation::ReorderShots(ReorderShots { id: payload.id.clone(), to_index: original_index })],
        None => Vec::new(),
    }
}
