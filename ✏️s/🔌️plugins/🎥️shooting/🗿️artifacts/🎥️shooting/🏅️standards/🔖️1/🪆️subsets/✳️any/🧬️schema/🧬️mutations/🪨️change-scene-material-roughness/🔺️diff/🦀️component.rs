//! 🔺 Diff constructor for `ChangeSceneMaterialRoughness`.

use super::mutation::ChangeSceneMaterialRoughness;
use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn diff(payload: &ChangeSceneMaterialRoughness, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if !payload.new_roughness.is_finite() || !(0.0..=1.0).contains(&payload.new_roughness) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Material roughness must be between 0 and 1, got {}.", payload.new_roughness), Vec::<String>::new());
    }
    if base.scene.material.roughness == payload.new_roughness {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Material roughness is already {}.", payload.new_roughness));
    }
    let mut scene = base.scene.clone();
    scene.material.roughness = payload.new_roughness;
    protocol::MutationOutcome::new(ShootingDiff { scene: Some(scene), ..Default::default() })
}
