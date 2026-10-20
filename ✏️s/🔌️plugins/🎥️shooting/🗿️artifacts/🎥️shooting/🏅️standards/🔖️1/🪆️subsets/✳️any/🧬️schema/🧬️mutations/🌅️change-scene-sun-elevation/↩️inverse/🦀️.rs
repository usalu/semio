//! ↩ Inverse constructor for `ChangeSceneSunElevation` — reconstructed from BASE state.

use super::ChangeSceneSunElevation;
use crate::ShootingSnapshot;

pub fn inverse<Op>(_payload: &ChangeSceneSunElevation, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_scene_sun_elevation::ChangeSceneSunElevation>,
{
    Ok((|| {
    vec![Op::from(ChangeSceneSunElevation { new_elevation: base.scene.sun.elevation })]

    })())
}
