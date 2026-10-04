//! ↩ Inverse constructor for `ChangeSceneSunEnabled` — reconstructed from BASE state.

use super::ChangeSceneSunEnabled;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(_payload: &ChangeSceneSunEnabled, base: &ShootingSnapshot) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ShootingMutation::ChangeSceneSunEnabled(ChangeSceneSunEnabled { new_enabled: base.scene.sun.enabled })]

    })())
}
