//! ↩ Inverse constructor for `ChangeSceneMaterialRoughness` — reconstructed from BASE state.

use super::ChangeSceneMaterialRoughness;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn inverse(_payload: &ChangeSceneMaterialRoughness, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::ChangeSceneMaterialRoughness(ChangeSceneMaterialRoughness { new_roughness: base.scene.material.roughness })]
}
