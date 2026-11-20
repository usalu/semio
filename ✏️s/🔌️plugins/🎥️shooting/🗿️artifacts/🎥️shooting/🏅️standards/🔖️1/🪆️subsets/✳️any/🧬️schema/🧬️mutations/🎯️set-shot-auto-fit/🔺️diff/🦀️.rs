//! 🔺️ Assigns or clears the affected shot's override.

use super::SetShotAutoFit;
use crate::{ShootingAssigned, ShootingShotPatch, ShootingSnapshot, diff::ShootingDiff};

pub fn diff(payload: &SetShotAutoFit, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    let Some(shot) = base.shots.iter().find(|shot| shot.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "Shot does not exist.", [payload.id.clone()]);
    };
    if shot.auto_fit == payload.auto_fit {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Shot auto-fit override is unchanged.");
    }
    protocol::MutationOutcome::new(ShootingDiff::shot_patches([(payload.id.clone(), ShootingShotPatch {
        auto_fit: Some(ShootingAssigned::new(payload.auto_fit)),
        ..Default::default()
    })]))
}
