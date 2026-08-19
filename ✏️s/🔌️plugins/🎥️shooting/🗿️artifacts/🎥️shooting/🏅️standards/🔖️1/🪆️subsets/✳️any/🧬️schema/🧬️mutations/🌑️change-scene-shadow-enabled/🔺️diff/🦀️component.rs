//! 🔺 Diff constructor for `ChangeSceneShadowEnabled`.

use super::mutation::ChangeSceneShadowEnabled;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::ShootingDiff;

pub async fn diff(payload: &ChangeSceneShadowEnabled, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if base.scene.shadow.enabled == payload.new_enabled {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Shadows are already {}.", if payload.new_enabled { "enabled" } else { "disabled" }));
    }
    let mut scene = base.scene.clone();
    scene.shadow.enabled = payload.new_enabled;
    protocol::MutationOutcome::new(ShootingDiff { scene: Some(scene), ..Default::default() })
}
