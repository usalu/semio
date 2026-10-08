//! 🔺️ Diff constructor for `SetSlab`: a sparse slab patch of exactly the provided fields that differ. A new slab type must exist, the
//! offset is finite and a slope (set or cleared) stays within [0, 89 degrees); providing only equal values, or no field, is a no-op.
//! Whole-list ruling: the holes are part of the slab's ONE planar region (see `set-slab-boundary`), so provided holes replace
//! the holes as one field.

use super::super::horizontal_rules::slope_fault;
use super::SetSlab;
use crate::{Entry, ModelDiff, ModelSnapshot, SlabPatch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetSlab, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(slab) = base.slabs.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Slab \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(slab_type) = payload.slab_type.as_ref().filter(|slab_type| !base.slab_types.contains_key(*slab_type)) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Slab type \"{slab_type}\" does not exist."), ["slab_type"]);
    }
    if payload.offset.is_some_and(|offset| !offset.is_finite()) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A slab offset must be a finite length.", ["offset"]);
    }
    if let Some(fault) = payload.slope.as_ref().and_then(|slope| slope.value.as_ref()).and_then(slope_fault) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["slope"]);
    }
    let patch = SlabPatch {
        slab_type: payload.slab_type.clone().filter(|slab_type| *slab_type != slab.slab_type),
        offset: payload.offset.filter(|offset| *offset != slab.offset),
        slope: payload.slope.clone().filter(|slope| slope.value != slab.slope),
        name: payload.name.clone().filter(|name| *name != slab.name),
        ..Default::default()
    };
    if patch == SlabPatch::default() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Slab \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::slabs(payload.id.clone(), Entry::Patched(patch)))
}
