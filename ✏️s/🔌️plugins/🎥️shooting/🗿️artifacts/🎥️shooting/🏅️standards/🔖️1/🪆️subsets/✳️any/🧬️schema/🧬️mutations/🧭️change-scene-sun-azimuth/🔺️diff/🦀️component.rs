//! 🔺 Diff constructor for `ChangeSceneSunAzimuth`.

use super::mutation::ChangeSceneSunAzimuth;
use crate::artifacts::shooting::diff::ShootingDiff;
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn diff(payload: &ChangeSceneSunAzimuth, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if !payload.new_azimuth.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Sun azimuth must be a finite number, got {}.", payload.new_azimuth), Vec::<String>::new());
    }
    if base.scene.sun.azimuth == payload.new_azimuth {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Sun azimuth is already {} degrees.", payload.new_azimuth));
    }
    let mut scene = base.scene.clone();
    scene.sun.azimuth = payload.new_azimuth;
    protocol::MutationOutcome::new(ShootingDiff { scene: Some(scene), ..Default::default() })
}
