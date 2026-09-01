//! ↩ Inverse constructor for `ChangeSceneAmbientIntensity` — reconstructed from BASE state.

use super::ChangeSceneAmbientIntensity;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn inverse(_payload: &ChangeSceneAmbientIntensity, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::ChangeSceneAmbientIntensity(ChangeSceneAmbientIntensity { new_intensity: base.scene.ambient.intensity })]
}
