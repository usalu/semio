//! ↩ Inverse constructor for `ChangeSceneSunIntensity` — reconstructed from BASE state.

use super::mutation::ChangeSceneSunIntensity;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;


pub async fn inverse(_payload: &ChangeSceneSunIntensity, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::ChangeSceneSunIntensity(ChangeSceneSunIntensity { new_intensity: base.scene.sun.intensity })]
}
