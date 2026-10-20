//! 🔺 Diff constructor for `ChangeSceneShadowEnabled`.

use super::ChangeSceneShadowEnabled;
use crate::diff::ShootingDiff;
use crate::ShootingSnapshot;

pub fn diff(payload: &ChangeSceneShadowEnabled, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if base.scene.shadow.enabled == payload.new_enabled {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Shadows are already {}.", if payload.new_enabled { "enabled" } else { "disabled" }));
    }
    protocol::MutationOutcome::new(ShootingDiff { scene: Some(crate::ShootingScenePatch { shadow_enabled: Some(payload.new_enabled), ..Default::default() }), ..Default::default() })
}
