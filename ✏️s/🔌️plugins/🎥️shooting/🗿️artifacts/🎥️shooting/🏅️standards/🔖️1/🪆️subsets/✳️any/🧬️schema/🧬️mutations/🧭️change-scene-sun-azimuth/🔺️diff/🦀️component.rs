//! 🔺 Diff constructor for `ChangeSceneSunAzimuth`.

use super::mutation::ChangeSceneSunAzimuth;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::ShootingDiff;

pub fn diff(payload: &ChangeSceneSunAzimuth, base: &ShootingSnapshot) -> ShootingDiff {
    let mut scene = base.scene.clone();
    scene.sun.azimuth = payload.new_azimuth;
    ShootingDiff { scene: Some(scene), ..Default::default() }
}
