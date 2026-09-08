//! ↩ Inverse constructor for `ChangeSceneSunElevation` — reconstructed from BASE state.

use super::ChangeSceneSunElevation;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(_payload: &ChangeSceneSunElevation, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::ChangeSceneSunElevation(ChangeSceneSunElevation { new_elevation: base.scene.sun.elevation })]
}
