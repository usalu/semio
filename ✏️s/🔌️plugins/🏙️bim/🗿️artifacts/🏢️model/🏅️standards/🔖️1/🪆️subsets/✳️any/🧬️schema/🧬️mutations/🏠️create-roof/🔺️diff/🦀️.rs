//! 🔺️ Diff constructor for `CreateRoof`: one created roof entry. The storey and the roof type must exist, the footprint is a valid
//! counter-clockwise loop, a pitched shape has pitches in (0, 89 degrees), the overhang is not negative and the base offset is finite.
//! The roof solid is inferred.

use super::super::elements;
use super::super::horizontal_rules::{loop_fault, overhang_fault, shape_fault};
use super::CreateRoof;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateRoof, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let roof = &payload.roof;
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !base.storeys.contains_key(&roof.storey) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{}\" does not exist.", roof.storey), ["roof", "storey"]);
    }
    if !base.roof_types.contains_key(&roof.roof_type) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Roof type \"{}\" does not exist.", roof.roof_type), ["roof", "roof_type"]);
    }
    if let Some(fault) = loop_fault(&roof.footprint) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["roof", "footprint"]);
    }
    if let Some(fault) = shape_fault(&roof.shape) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["roof", "shape"]);
    }
    if let Some(fault) = overhang_fault(roof.overhang) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["roof", "overhang"]);
    }
    if !roof.base_offset.is_finite() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A roof base offset must be a finite length.", ["roof", "base_offset"]);
    }
    MutationOutcome::new(ModelDiff::roofs(payload.id.clone(), Entry::Created(roof.clone())))
}
