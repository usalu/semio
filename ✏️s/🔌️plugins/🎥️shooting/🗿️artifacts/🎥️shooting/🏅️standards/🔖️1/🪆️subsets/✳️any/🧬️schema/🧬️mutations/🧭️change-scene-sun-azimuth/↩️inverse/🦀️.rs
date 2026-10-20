//! ↩ Inverse constructor for `ChangeSceneSunAzimuth` — reconstructed from BASE state.

use super::ChangeSceneSunAzimuth;
use crate::ShootingSnapshot;

pub fn inverse<Op>(_payload: &ChangeSceneSunAzimuth, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_scene_sun_azimuth::ChangeSceneSunAzimuth>,
{
    Ok((|| {
    vec![Op::from(ChangeSceneSunAzimuth { new_azimuth: base.scene.sun.azimuth })]

    })())
}
