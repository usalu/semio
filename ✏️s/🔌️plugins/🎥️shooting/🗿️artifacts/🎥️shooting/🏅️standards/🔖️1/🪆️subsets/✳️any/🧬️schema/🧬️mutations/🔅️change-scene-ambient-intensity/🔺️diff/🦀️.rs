//! 🔺 Diff constructor for `ChangeSceneAmbientIntensity`.

use super::ChangeSceneAmbientIntensity;
use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn diff(payload: &ChangeSceneAmbientIntensity, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if !payload.new_intensity.is_finite() || payload.new_intensity < 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Ambient intensity must be a non-negative finite number, got {}.", payload.new_intensity), Vec::<String>::new());
    }
    if base.scene.ambient.intensity == payload.new_intensity {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Ambient intensity is already {}.", payload.new_intensity));
    }
    let mut scene = base.scene.clone();
    scene.ambient.intensity = payload.new_intensity;
    protocol::MutationOutcome::new(ShootingDiff { scene: Some(scene), ..Default::default() })
}
