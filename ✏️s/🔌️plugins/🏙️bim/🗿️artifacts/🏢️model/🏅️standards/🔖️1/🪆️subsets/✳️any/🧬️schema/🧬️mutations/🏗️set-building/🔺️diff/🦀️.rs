//! 🔺️ Diff constructor for `SetBuilding`: a sparse building patch of exactly the provided fields; the site is never touched, a
//! building is not re-parented. Origin, rotation and elevation are finite; a patch that restates the current values is a
//! `mutation.no-op`. Nothing downstream is written: the absolute elevation of every storey of the building follows the building
//! elevation by inference (`storey-levels`).

use super::SetBuilding;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetBuilding, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(building) = base.buildings.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Building \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.origin.is_some_and(|point| !(point.x.is_finite() && point.y.is_finite())) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A building origin must be a finite point.", ["origin"]);
    }
    if payload.rotation.is_some_and(|value| !value.is_finite()) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A building rotation must be a finite angle.", ["rotation"]);
    }
    if payload.elevation.is_some_and(|value| !value.is_finite()) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A building elevation must be a finite length.", ["elevation"]);
    }
    let patch = payload.patch().minimal(building);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Building \"{}\" already holds these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::buildings(payload.id.clone(), Entry::Patched(patch)))
}
