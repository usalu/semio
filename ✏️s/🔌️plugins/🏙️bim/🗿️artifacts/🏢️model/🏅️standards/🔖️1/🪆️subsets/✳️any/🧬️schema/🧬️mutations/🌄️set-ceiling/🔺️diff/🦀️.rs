//! 🔺️ Diff constructor for `SetCeiling`: a sparse ceiling patch of exactly the provided fields that differ. A new ceiling type must exist, the
//! drop is finite and a slope (set or cleared) stays within [0, 89 degrees); providing only equal values, or no field, is a no-op.

use super::super::horizontal_rules::slope_fault;
use super::SetCeiling;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetCeiling, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(ceiling) = base.ceilings.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Ceiling \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(ceiling_type) = payload.ceiling_type.as_ref().filter(|ceiling_type| !base.ceiling_types.contains_key(*ceiling_type)) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Ceiling type \"{ceiling_type}\" does not exist."), ["ceiling_type"]);
    }
    if payload.offset.is_some_and(|offset| !offset.is_finite()) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A ceiling drop must be a finite length.", ["offset"]);
    }
    if let Some(fault) = payload.slope.as_ref().and_then(|slope| slope.value.as_ref()).and_then(slope_fault) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["slope"]);
    }
    let change = payload.patch().minimal(ceiling);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Ceiling \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::ceilings(payload.id.clone(), Entry::Patched(change)))
}
