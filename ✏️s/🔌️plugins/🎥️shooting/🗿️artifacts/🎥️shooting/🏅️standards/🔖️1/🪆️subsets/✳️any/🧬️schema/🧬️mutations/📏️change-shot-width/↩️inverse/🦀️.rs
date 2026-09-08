//! ↩ Inverse constructor for `ChangeShotWidth` — reconstructed from BASE state.

use super::ChangeShotWidth;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(payload: &ChangeShotWidth, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.shots.iter().find(|shot| shot.id == payload.id) {
        Some(shot) => vec![ShootingMutation::ChangeShotWidth(ChangeShotWidth { id: payload.id.clone(), new_width: shot.width })],
        None => Vec::new(),
    }
}
