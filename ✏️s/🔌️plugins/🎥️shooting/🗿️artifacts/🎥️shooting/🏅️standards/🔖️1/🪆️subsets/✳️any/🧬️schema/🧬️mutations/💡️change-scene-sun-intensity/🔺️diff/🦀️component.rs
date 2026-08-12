//! 🔺 Diff constructor for `ChangeSceneSunIntensity`.

use super::mutation::ChangeSceneSunIntensity;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::ShootingDiff;

pub fn diff(payload: &ChangeSceneSunIntensity, base: &ShootingSnapshot) -> ShootingDiff {
    let mut scene = base.scene.clone();
    scene.sun.intensity = payload.new_intensity;
    ShootingDiff { scene: Some(scene), ..Default::default() }
}
