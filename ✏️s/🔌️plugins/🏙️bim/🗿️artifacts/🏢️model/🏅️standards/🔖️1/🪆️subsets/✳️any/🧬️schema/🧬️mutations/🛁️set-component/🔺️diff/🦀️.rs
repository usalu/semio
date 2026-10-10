//! 🔺️ Diff constructor for `SetComponent`: a sparse component patch of exactly the provided fields that differ. The component that results must break none of the create rules; a mounted component
//! stands on the storey of its wall, so it cannot change its storey unless it changes its wall with it, and a component that changes its family keeps only overrides the new family can evaluate. Moving to
//! another storey keeps the elevation. Providing only equal values, or no field, is a no-op.

use super::super::component_rules;
use super::SetComponent;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetComponent, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.components.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Component \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let next = payload.patch().write(record);
    if let Some(wall) = next.host.as_ref().filter(|wall| record.host.as_ref() == Some(*wall) && next.storey != record.storey) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, format!("Component \"{}\" is mounted on wall \"{wall}\" and stands on its storey.", payload.id), ["storey"]);
    }
    if let Some(fault) = component_rules::component_fault(base, &next) {
        return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(None));
    }
    if next.family != record.family {
        if let Some(fault) = component_rules::family_swap_fault(base, &payload.id, &next.family) {
            return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(None));
        }
    }
    let change = payload.patch().minimal(record);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Component \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::components(payload.id.clone(), Entry::Patched(change)))
}
