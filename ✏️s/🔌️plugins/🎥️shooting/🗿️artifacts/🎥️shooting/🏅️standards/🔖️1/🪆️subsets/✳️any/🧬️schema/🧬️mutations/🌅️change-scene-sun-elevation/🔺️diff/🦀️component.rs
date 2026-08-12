//! 🔺 Diff constructor for `ChangeSceneSunElevation`.

use super::mutation::ChangeSceneSunElevation;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::ShootingDiff;

pub fn diff(payload: &ChangeSceneSunElevation, base: &ShootingSnapshot) -> ShootingDiff {
    let mut scene = base.scene.clone();
    scene.sun.elevation = payload.new_elevation;
    ShootingDiff { scene: Some(scene), ..Default::default() }
}
