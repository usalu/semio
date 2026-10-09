//! 🔺️ Diff constructor for `CreateCeiling`: one created ceiling entry. The storey and the ceiling type must exist, the boundary and every hole
//! are valid counter-clockwise loops with the holes strictly inside, the drop is finite and a slope stays within [0, 89 degrees).
//! Area, volume, the sloped solid and the clear height of the rooms below are inferred.

use super::super::elements;
use super::super::horizontal_rules::{holes_fault, loop_fault, slope_fault};
use super::CreateCeiling;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateCeiling, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let ceiling = &payload.ceiling;
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !base.storeys.contains_key(&ceiling.storey) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{}\" does not exist.", ceiling.storey), ["ceiling", "storey"]);
    }
    if !base.ceiling_types.contains_key(&ceiling.ceiling_type) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Ceiling type \"{}\" does not exist.", ceiling.ceiling_type), ["ceiling", "ceiling_type"]);
    }
    if let Some(fault) = loop_fault(&ceiling.boundary) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["ceiling", "boundary"]);
    }
    if let Some(fault) = holes_fault(&ceiling.boundary, &ceiling.holes) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["ceiling", "holes"]);
    }
    if !ceiling.offset.is_finite() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A ceiling drop must be a finite length.", ["ceiling", "offset"]);
    }
    if let Some(fault) = ceiling.slope.as_ref().and_then(slope_fault) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["ceiling", "slope"]);
    }
    MutationOutcome::new(ModelDiff::ceilings(payload.id.clone(), Entry::Created(ceiling.clone())))
}
