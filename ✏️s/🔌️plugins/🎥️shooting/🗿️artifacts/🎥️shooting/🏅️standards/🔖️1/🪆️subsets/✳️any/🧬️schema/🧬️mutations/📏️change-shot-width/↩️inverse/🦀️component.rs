//! ↩ Inverse constructor for `ChangeShotWidth` — reconstructed from BASE state.

use super::mutation::ChangeShotWidth;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;


pub fn inverse(payload: &ChangeShotWidth, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.shots.iter().find(|shot| shot.id == payload.id) {
        Some(shot) => vec![ShootingMutation::ChangeShotWidth(ChangeShotWidth { id: payload.id.clone(), new_width: shot.width })],
        None => Vec::new(),
    }
}
