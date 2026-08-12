//! 🔺 Diff constructor for `ChangeSceneSunEnabled`.

use super::mutation::ChangeSceneSunEnabled;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::ShootingDiff;

pub fn diff(payload: &ChangeSceneSunEnabled, base: &ShootingSnapshot) -> ShootingDiff {
    let mut scene = base.scene.clone();
    scene.sun.enabled = payload.new_enabled;
    ShootingDiff { scene: Some(scene), ..Default::default() }
}
