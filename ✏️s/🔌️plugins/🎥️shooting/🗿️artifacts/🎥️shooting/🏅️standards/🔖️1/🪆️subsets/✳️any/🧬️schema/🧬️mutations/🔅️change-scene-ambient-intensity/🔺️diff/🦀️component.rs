//! 🔺 Diff constructor for `ChangeSceneAmbientIntensity`.

use super::mutation::ChangeSceneAmbientIntensity;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::ShootingDiff;

pub fn diff(payload: &ChangeSceneAmbientIntensity, base: &ShootingSnapshot) -> ShootingDiff {
    let mut scene = base.scene.clone();
    scene.ambient.intensity = payload.new_intensity;
    ShootingDiff { scene: Some(scene), ..Default::default() }
}
