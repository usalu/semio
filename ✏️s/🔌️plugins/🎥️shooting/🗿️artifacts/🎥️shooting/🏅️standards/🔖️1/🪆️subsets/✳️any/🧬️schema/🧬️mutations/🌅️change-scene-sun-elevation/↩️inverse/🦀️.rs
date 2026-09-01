//! ↩ Inverse constructor for `ChangeSceneSunElevation` — reconstructed from BASE state.

use super::ChangeSceneSunElevation;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn inverse(_payload: &ChangeSceneSunElevation, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::ChangeSceneSunElevation(ChangeSceneSunElevation { new_elevation: base.scene.sun.elevation })]
}
