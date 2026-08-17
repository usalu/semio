//! ↩ Inverse constructor for `ChangeSceneSunElevation` — reconstructed from BASE state.

use super::mutation::ChangeSceneSunElevation;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;


pub fn inverse(_payload: &ChangeSceneSunElevation, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::ChangeSceneSunElevation(ChangeSceneSunElevation { new_elevation: base.scene.sun.elevation })]
}
