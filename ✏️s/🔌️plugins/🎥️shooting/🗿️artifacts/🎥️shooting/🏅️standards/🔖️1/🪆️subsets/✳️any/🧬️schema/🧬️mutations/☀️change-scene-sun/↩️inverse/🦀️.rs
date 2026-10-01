//! ↩ Inverse constructor for `ChangeSceneSunEnabled` — reconstructed from BASE state.

use super::ChangeSceneSunEnabled;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(_payload: &ChangeSceneSunEnabled, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::ChangeSceneSunEnabled(ChangeSceneSunEnabled { new_enabled: base.scene.sun.enabled })]
}
