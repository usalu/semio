//! 🔺 Diff constructor for `ChangeSceneSunIntensity`.

use super::mutation::ChangeSceneSunIntensity;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::ShootingDiff;

pub async fn diff(payload: &ChangeSceneSunIntensity, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if !payload.new_intensity.is_finite() || payload.new_intensity < 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Sun intensity must be a non-negative finite number, got {}.", payload.new_intensity), Vec::<String>::new());
    }
    if base.scene.sun.intensity == payload.new_intensity {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Sun intensity is already {}.", payload.new_intensity));
    }
    let mut scene = base.scene.clone();
    scene.sun.intensity = payload.new_intensity;
    protocol::MutationOutcome::new(ShootingDiff { scene: Some(scene), ..Default::default() })
}
