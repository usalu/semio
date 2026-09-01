//! ↩ Inverse constructor for `ChangeSceneSunAzimuth` — reconstructed from BASE state.

use super::ChangeSceneSunAzimuth;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn inverse(_payload: &ChangeSceneSunAzimuth, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::ChangeSceneSunAzimuth(ChangeSceneSunAzimuth { new_azimuth: base.scene.sun.azimuth })]
}
