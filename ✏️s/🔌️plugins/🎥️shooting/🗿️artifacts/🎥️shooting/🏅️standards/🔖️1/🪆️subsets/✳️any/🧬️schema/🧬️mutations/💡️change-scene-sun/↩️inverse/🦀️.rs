//! ↩ Inverse constructor for `ChangeSceneSunIntensity` — reconstructed from BASE state.

use super::ChangeSceneSunIntensity;
use crate::ShootingSnapshot;

pub fn inverse<Op>(_payload: &ChangeSceneSunIntensity, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_scene_sun_intensity::ChangeSceneSunIntensity>,
{
    Ok((|| {
    vec![Op::from(ChangeSceneSunIntensity { new_intensity: base.scene.sun.intensity })]

    })())
}
