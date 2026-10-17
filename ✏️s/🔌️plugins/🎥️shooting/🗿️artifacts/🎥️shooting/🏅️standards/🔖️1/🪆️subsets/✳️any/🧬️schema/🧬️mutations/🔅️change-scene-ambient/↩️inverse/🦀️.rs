//! ↩ Inverse constructor for `ChangeSceneAmbientIntensity` — reconstructed from BASE state.

use super::ChangeSceneAmbientIntensity;
use crate::ShootingSnapshot;

pub fn inverse<Op>(_payload: &ChangeSceneAmbientIntensity, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_scene_ambient_intensity::ChangeSceneAmbientIntensity>,
{
    Ok((|| {
    vec![Op::from(ChangeSceneAmbientIntensity { new_intensity: base.scene.ambient.intensity })]

    })())
}
