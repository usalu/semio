//! ↩ Inverse constructor for `ChangeShotHeight` — reconstructed from BASE state.

use super::ChangeShotHeight;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn inverse(payload: &ChangeShotHeight, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.shots.iter().find(|shot| shot.id == payload.id) {
        Some(shot) => vec![ShootingMutation::ChangeShotHeight(ChangeShotHeight { id: payload.id.clone(), new_height: shot.height })],
        None => Vec::new(),
    }
}
