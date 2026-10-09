//! 🔺️ Diff constructor for `SetZone`: a sparse zone patch of exactly the provided fields that differ from the base. The occupancy density
//! stays a finite, non-negative number of persons per square metre; providing only equal values is a no-op.

use super::SetZone;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetZone, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(zone) = base.zones.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Zone \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if payload.occupancy_density.is_some_and(|density| !(density.is_finite() && density >= 0.0)) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "An occupancy density is a finite number of persons per square metre, zero or more.", ["occupancy_density"]);
    }
    let patch = payload.patch().minimal(zone);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Zone \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::zones(payload.id.clone(), Entry::Patched(patch)))
}
