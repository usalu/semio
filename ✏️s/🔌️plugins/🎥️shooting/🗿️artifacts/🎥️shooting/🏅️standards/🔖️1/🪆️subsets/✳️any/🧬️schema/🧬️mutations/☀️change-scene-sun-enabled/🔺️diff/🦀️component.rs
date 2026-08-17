//! 🔺 Diff constructor for `ChangeSceneSunEnabled`.

use super::mutation::ChangeSceneSunEnabled;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::ShootingDiff;

pub fn diff(payload: &ChangeSceneSunEnabled, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if base.scene.sun.enabled == payload.new_enabled {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Sun is already {}.", if payload.new_enabled { "enabled" } else { "disabled" }));
    }
    let mut scene = base.scene.clone();
    scene.sun.enabled = payload.new_enabled;
    protocol::MutationOutcome::new(ShootingDiff { scene: Some(scene), ..Default::default() })
}
