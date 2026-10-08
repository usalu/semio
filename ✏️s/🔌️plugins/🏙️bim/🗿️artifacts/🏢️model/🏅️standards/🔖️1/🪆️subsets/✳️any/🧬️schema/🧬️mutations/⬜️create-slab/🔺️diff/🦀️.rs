//! 🔺️ Diff constructor for `CreateSlab`: one created slab entry. The storey and the slab type must exist, the boundary and every hole
//! are valid counter-clockwise loops with the holes strictly inside, the offset is finite and a slope stays within [0, 89 degrees).
//! Area, volume and the sloped solid are inferred.

use super::super::elements;
use super::super::horizontal_rules::{holes_fault, loop_fault, slope_fault};
use super::CreateSlab;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateSlab, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let slab = &payload.slab;
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !base.storeys.contains_key(&slab.storey) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{}\" does not exist.", slab.storey), ["slab", "storey"]);
    }
    if !base.slab_types.contains_key(&slab.slab_type) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Slab type \"{}\" does not exist.", slab.slab_type), ["slab", "slab_type"]);
    }
    if let Some(fault) = loop_fault(&slab.boundary) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["slab", "boundary"]);
    }
    if let Some(fault) = holes_fault(&slab.boundary, &slab.holes) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["slab", "holes"]);
    }
    if !slab.offset.is_finite() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A slab offset must be a finite length.", ["slab", "offset"]);
    }
    if let Some(fault) = slab.slope.as_ref().and_then(slope_fault) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["slab", "slope"]);
    }
    MutationOutcome::new(ModelDiff::slabs(payload.id.clone(), Entry::Created(slab.clone())))
}
