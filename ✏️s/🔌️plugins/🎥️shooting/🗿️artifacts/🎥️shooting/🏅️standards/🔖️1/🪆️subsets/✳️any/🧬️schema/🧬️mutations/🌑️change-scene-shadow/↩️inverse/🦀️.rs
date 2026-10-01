//! ↩ Inverse constructor for `ChangeSceneShadowEnabled` — reconstructed from BASE state.

use super::ChangeSceneShadowEnabled;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(_payload: &ChangeSceneShadowEnabled, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::ChangeSceneShadowEnabled(ChangeSceneShadowEnabled { new_enabled: base.scene.shadow.enabled })]
}
