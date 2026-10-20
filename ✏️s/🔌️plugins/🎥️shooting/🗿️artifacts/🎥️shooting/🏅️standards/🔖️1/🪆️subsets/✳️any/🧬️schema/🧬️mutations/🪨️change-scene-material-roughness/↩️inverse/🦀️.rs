//! ↩ Inverse constructor for `ChangeSceneMaterialRoughness` — reconstructed from BASE state.

use super::ChangeSceneMaterialRoughness;
use crate::ShootingSnapshot;

pub fn inverse<Op>(_payload: &ChangeSceneMaterialRoughness, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_scene_material_roughness::ChangeSceneMaterialRoughness>,
{
    Ok((|| {
    vec![Op::from(ChangeSceneMaterialRoughness { new_roughness: base.scene.material.roughness })]

    })())
}
