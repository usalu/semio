//! ↩ Inverse constructor for `ChangeSceneShadowEnabled` — reconstructed from BASE state.

use super::ChangeSceneShadowEnabled;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(_payload: &ChangeSceneShadowEnabled, base: &ShootingSnapshot) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ShootingMutation::ChangeSceneShadowEnabled(ChangeSceneShadowEnabled { new_enabled: base.scene.shadow.enabled })]

    })())
}
