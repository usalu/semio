//! ↩ Inverse constructor for `ChangeSceneSunEnabled` — reconstructed from BASE state.

use super::ChangeSceneSunEnabled;
use crate::ShootingSnapshot;

pub fn inverse<Op>(_payload: &ChangeSceneSunEnabled, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_scene_sun_enabled::ChangeSceneSunEnabled>,
{
    Ok((|| {
    vec![Op::from(ChangeSceneSunEnabled { new_enabled: base.scene.sun.enabled })]

    })())
}
