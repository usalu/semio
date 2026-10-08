//! 🔺️ Diff constructor for `CreateStorey`: one created storey entry; the building must exist, the height must be positive,
//! an authored plan cut height must be positive and the level index must be free within the building.

use super::super::elements;
use super::CreateStorey;
use crate::{cut_height_problem, Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateStorey, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let storey = &payload.storey;
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !base.buildings.contains_key(&storey.building) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Building \"{}\" does not exist.", storey.building), ["storey", "building"]);
    }
    if !(storey.height.is_finite() && storey.height > 0.0) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A storey height must be a positive length.", ["storey", "height"]);
    }
    if let Some(message) = storey.cut_height.and_then(cut_height_problem) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, message, ["storey", "cut_height"]);
    }
    if base.storeys.values().any(|other| other.building == storey.building && other.level == storey.level) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, format!("Level {} is already taken in building \"{}\".", storey.level, storey.building), ["storey", "level"]);
    }
    MutationOutcome::new(ModelDiff::storeys(payload.id.clone(), Entry::Created(storey.clone())))
}
