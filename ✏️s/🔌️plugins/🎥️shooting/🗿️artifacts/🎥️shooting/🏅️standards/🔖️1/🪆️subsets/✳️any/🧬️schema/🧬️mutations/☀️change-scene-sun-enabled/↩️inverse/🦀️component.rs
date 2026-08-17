//! ↩ Inverse constructor for `ChangeSceneSunEnabled` — reconstructed from BASE state.

use super::mutation::ChangeSceneSunEnabled;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;


pub fn inverse(_payload: &ChangeSceneSunEnabled, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    vec![ShootingMutation::ChangeSceneSunEnabled(ChangeSceneSunEnabled { new_enabled: base.scene.sun.enabled })]
}
