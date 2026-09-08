//! ↩ Inverse constructor for `ChangeShotShape` — reconstructed from BASE state.

use super::ChangeShotShape;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(payload: &ChangeShotShape, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.shots.iter().find(|shot| shot.id == payload.id) {
        Some(shot) => vec![ShootingMutation::ChangeShotShape(ChangeShotShape { id: payload.id.clone(), new_shape: shot.shape.clone() })],
        None => Vec::new(),
    }
}
