//! 🔺 Diff constructor for `ChangeSceneShadowEnabled`.

use super::mutation::ChangeSceneShadowEnabled;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::ShootingDiff;

pub fn diff(payload: &ChangeSceneShadowEnabled, base: &ShootingSnapshot) -> ShootingDiff {
    let mut scene = base.scene.clone();
    scene.shadow.enabled = payload.new_enabled;
    ShootingDiff { scene: Some(scene), ..Default::default() }
}
