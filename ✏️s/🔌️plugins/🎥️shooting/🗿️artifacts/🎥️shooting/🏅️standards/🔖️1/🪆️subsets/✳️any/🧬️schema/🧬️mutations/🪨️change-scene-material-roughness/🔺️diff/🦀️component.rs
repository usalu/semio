//! 🔺 Diff constructor for `ChangeSceneMaterialRoughness`.

use super::mutation::ChangeSceneMaterialRoughness;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::ShootingDiff;

pub fn diff(payload: &ChangeSceneMaterialRoughness, base: &ShootingSnapshot) -> ShootingDiff {
    let mut scene = base.scene.clone();
    scene.material.roughness = payload.new_roughness;
    ShootingDiff { scene: Some(scene), ..Default::default() }
}
