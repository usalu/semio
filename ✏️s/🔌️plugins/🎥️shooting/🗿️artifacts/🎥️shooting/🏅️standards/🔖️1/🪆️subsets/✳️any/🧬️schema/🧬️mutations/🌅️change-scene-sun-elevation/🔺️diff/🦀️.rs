//! 🔺 Diff constructor for `ChangeSceneSunElevation`.

use super::ChangeSceneSunElevation;
use crate::diff::ShootingDiff;
use crate::ShootingSnapshot;

pub fn diff(payload: &ChangeSceneSunElevation, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if !payload.new_elevation.is_finite() || !(-90.0..=90.0).contains(&payload.new_elevation) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Sun elevation must be between -90 and 90 degrees, got {}.", payload.new_elevation), Vec::<String>::new());
    }
    if base.scene.sun.elevation == payload.new_elevation {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Sun elevation is already {} degrees.", payload.new_elevation));
    }
    protocol::MutationOutcome::new(ShootingDiff { scene: Some(crate::ShootingScenePatch { sun_elevation: Some(payload.new_elevation), ..Default::default() }), ..Default::default() })
}
