//! 🔺️ Diff constructor for `SetRoofShape`: a sparse roof patch of exactly the provided fields that differ. A pitched shape has pitches
//! in (0, 89 degrees), the overhang is not negative and the base offset is finite; providing only equal values, or no field, is a no-op.

use super::super::horizontal_rules::{overhang_fault, shape_fault};
use super::SetRoofShape;
use crate::{Entry, ModelDiff, ModelSnapshot, RoofPatch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetRoofShape, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(roof) = base.roofs.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Roof \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(fault) = payload.shape.as_ref().and_then(shape_fault) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["shape"]);
    }
    if let Some(fault) = payload.overhang.and_then(overhang_fault) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["overhang"]);
    }
    if payload.base_offset.is_some_and(|offset| !offset.is_finite()) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A roof base offset must be a finite length.", ["base_offset"]);
    }
    let patch = RoofPatch {
        shape: payload.shape.clone().filter(|shape| *shape != roof.shape),
        overhang: payload.overhang.filter(|overhang| *overhang != roof.overhang),
        base_offset: payload.base_offset.filter(|offset| *offset != roof.base_offset),
        ..Default::default()
    };
    if patch == RoofPatch::default() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Roof \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::roofs(payload.id.clone(), Entry::Patched(patch)))
}
