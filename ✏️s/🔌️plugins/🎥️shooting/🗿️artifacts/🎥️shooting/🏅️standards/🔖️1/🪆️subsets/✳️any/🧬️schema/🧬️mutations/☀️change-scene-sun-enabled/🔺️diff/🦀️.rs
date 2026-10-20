//! 🔺 Diff constructor for `ChangeSceneSunEnabled`.

use super::ChangeSceneSunEnabled;
use crate::diff::ShootingDiff;
use crate::ShootingSnapshot;

pub fn diff(payload: &ChangeSceneSunEnabled, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if base.scene.sun.enabled == payload.new_enabled {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Sun is already {}.", if payload.new_enabled { "enabled" } else { "disabled" }));
    }
    protocol::MutationOutcome::new(ShootingDiff { scene: Some(crate::ShootingScenePatch { sun_enabled: Some(payload.new_enabled), ..Default::default() }), ..Default::default() })
}
