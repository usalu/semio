//! 🔺 Diff constructor for `ChangeSceneSunElevation`.

use super::ChangeSceneSunElevation;
use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn diff(payload: &ChangeSceneSunElevation, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if !payload.new_elevation.is_finite() || !(-90.0..=90.0).contains(&payload.new_elevation) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Sun elevation must be between -90 and 90 degrees, got {}.", payload.new_elevation), Vec::<String>::new());
    }
    if base.scene.sun.elevation == payload.new_elevation {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Sun elevation is already {} degrees.", payload.new_elevation));
    }
    let mut scene = base.scene.clone();
    scene.sun.elevation = payload.new_elevation;
    protocol::MutationOutcome::new(ShootingDiff { scene: Some(scene), ..Default::default() })
}
