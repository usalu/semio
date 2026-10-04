//! ↩ Inverse constructor for `ChangeSceneMaterialRoughness` — reconstructed from BASE state.

use super::ChangeSceneMaterialRoughness;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(_payload: &ChangeSceneMaterialRoughness, base: &ShootingSnapshot) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ShootingMutation::ChangeSceneMaterialRoughness(ChangeSceneMaterialRoughness { new_roughness: base.scene.material.roughness })]

    })())
}
