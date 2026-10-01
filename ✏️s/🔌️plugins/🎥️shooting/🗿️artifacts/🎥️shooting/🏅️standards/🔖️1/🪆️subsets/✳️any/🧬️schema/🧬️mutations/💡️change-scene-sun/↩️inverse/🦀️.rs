//! ↩ Inverse constructor for `ChangeSceneSunIntensity` — reconstructed from BASE state.

use super::ChangeSceneSunIntensity;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(_payload: &ChangeSceneSunIntensity, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::ChangeSceneSunIntensity(ChangeSceneSunIntensity { new_intensity: base.scene.sun.intensity })]
}
