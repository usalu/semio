//! ↩ Inverse constructor for `ChangeSceneShadowEnabled` — reconstructed from BASE state.

use super::mutation::ChangeSceneShadowEnabled;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;


pub fn inverse(payload: &ChangeSceneShadowEnabled, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::ChangeSceneShadowEnabled(ChangeSceneShadowEnabled { new_enabled: base.scene.shadow.enabled })]
}
