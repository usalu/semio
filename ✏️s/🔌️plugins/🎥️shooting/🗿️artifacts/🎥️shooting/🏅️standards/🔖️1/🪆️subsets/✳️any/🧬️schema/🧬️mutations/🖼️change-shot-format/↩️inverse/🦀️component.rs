//! ↩ Inverse constructor for `ChangeShotFormat` — reconstructed from BASE state.

use super::mutation::ChangeShotFormat;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;


pub fn inverse(payload: &ChangeShotFormat, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.shots.iter().find(|shot| shot.id == payload.id) {
        Some(shot) => vec![ShootingMutation::ChangeShotFormat(ChangeShotFormat { id: payload.id.clone(), new_format: shot.format.clone() })],
        None => Vec::new(),
    }
}
