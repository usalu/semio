//! 🔺️ Diff constructor for `SetGridLine`: a sparse grid line patch of exactly the provided fields; the building is never touched.
//! The label is non-blank and unique within the building (the line itself excluded), the resulting end points are finite and
//! apart. A patch that restates the current values is a `mutation.no-op`.

use super::SetGridLine;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetGridLine, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(line) = base.grids.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Grid line \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.label.as_ref().is_some_and(|label| label.trim().is_empty()) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A grid line label must not be blank.", ["label"]);
    }
    let start = payload.start.unwrap_or(line.start);
    let end = payload.end.unwrap_or(line.end);
    if ![start.x, start.y, end.x, end.y].iter().all(|value| value.is_finite()) || start == end {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A grid line needs finite end points and a length.", [if payload.end.is_some() { "end" } else { "start" }]);
    }
    if let Some(label) = &payload.label {
        if base.grids.iter().any(|(other, row)| other != &payload.id && row.building == line.building && &row.label == label) {
            return MutationOutcome::refuse(OutcomeCode::Invariant, format!("Building \"{}\" already has a grid line labelled \"{label}\".", line.building), ["label"]);
        }
    }
    let patch = payload.patch().minimal(line);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Grid line \"{}\" already holds these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::grids(payload.id.clone(), Entry::Patched(patch)))
}
