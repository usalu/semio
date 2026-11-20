//! 🔺️ Builds a drawing assignment diff from the current snapshot.

use super::SetShotDrawing;
use crate::{ShootingShotPatch, ShootingSnapshot, diff::ShootingDiff};

pub fn diff(payload: &SetShotDrawing, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    let Some(shot) = base.shots.iter().find(|shot| shot.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Shot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let drawing = payload.patch.apply(shot.drawing.as_ref());
    if drawing.as_ref().is_some_and(|drawing| !drawing.is_valid()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Shot \"{}\" has invalid drawing settings.", payload.id), [payload.id.clone()]);
    }
    if shot.drawing == drawing {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Shot \"{}\" already has these drawing settings.", payload.id));
    }
    protocol::MutationOutcome::new(ShootingDiff::shot_patches([(payload.id.clone(), ShootingShotPatch {
        drawing: Some(payload.patch.clone()),
        ..Default::default()
    })]))
}
