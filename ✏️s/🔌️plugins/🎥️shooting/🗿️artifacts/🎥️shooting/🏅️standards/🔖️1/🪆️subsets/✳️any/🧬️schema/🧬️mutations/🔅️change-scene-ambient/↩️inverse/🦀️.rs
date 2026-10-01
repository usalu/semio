//! ↩ Inverse constructor for `ChangeSceneAmbientIntensity` — reconstructed from BASE state.

use super::ChangeSceneAmbientIntensity;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(_payload: &ChangeSceneAmbientIntensity, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::ChangeSceneAmbientIntensity(ChangeSceneAmbientIntensity { new_intensity: base.scene.ambient.intensity })]
}
