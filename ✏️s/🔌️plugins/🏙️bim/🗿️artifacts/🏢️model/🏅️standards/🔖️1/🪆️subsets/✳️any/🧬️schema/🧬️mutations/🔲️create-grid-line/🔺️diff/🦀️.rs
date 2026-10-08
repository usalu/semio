//! 🔺️ Diff constructor for `CreateGridLine`: one created grid line entry. The id must be free and the building must exist; the
//! label is non-blank and unique within the building, the line has finite end points and a length.

use super::super::elements;
use super::CreateGridLine;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateGridLine, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let line = &payload.grid_line;
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !base.buildings.contains_key(&line.building) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Building \"{}\" does not exist.", line.building), ["grid_line", "building"]);
    }
    if line.label.trim().is_empty() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A grid line label must not be blank.", ["grid_line", "label"]);
    }
    if ![line.start.x, line.start.y, line.end.x, line.end.y].iter().all(|value| value.is_finite()) || line.start == line.end {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A grid line needs finite end points and a length.", ["grid_line", "end"]);
    }
    if base.grids.values().any(|row| row.building == line.building && row.label == line.label) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, format!("Building \"{}\" already has a grid line labelled \"{}\".", line.building, line.label), ["grid_line", "label"]);
    }
    MutationOutcome::new(ModelDiff::grids(payload.id.clone(), Entry::Created(line.clone())))
}
