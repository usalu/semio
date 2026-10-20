//! 🔺 Diff constructor for `ChangeSceneSunAzimuth`.

use super::ChangeSceneSunAzimuth;
use crate::diff::ShootingDiff;
use crate::ShootingSnapshot;

pub fn diff(payload: &ChangeSceneSunAzimuth, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if !payload.new_azimuth.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Sun azimuth must be a finite number, got {}.", payload.new_azimuth), Vec::<String>::new());
    }
    if base.scene.sun.azimuth == payload.new_azimuth {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Sun azimuth is already {} degrees.", payload.new_azimuth));
    }
    protocol::MutationOutcome::new(ShootingDiff { scene: Some(crate::ShootingScenePatch { sun_azimuth: Some(payload.new_azimuth), ..Default::default() }), ..Default::default() })
}
