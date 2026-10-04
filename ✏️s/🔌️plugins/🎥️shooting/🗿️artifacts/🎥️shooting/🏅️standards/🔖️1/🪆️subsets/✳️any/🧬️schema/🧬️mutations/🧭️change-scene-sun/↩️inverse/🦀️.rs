//! ↩ Inverse constructor for `ChangeSceneSunAzimuth` — reconstructed from BASE state.

use super::ChangeSceneSunAzimuth;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(_payload: &ChangeSceneSunAzimuth, base: &ShootingSnapshot) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ShootingMutation::ChangeSceneSunAzimuth(ChangeSceneSunAzimuth { new_azimuth: base.scene.sun.azimuth })]

    })())
}
