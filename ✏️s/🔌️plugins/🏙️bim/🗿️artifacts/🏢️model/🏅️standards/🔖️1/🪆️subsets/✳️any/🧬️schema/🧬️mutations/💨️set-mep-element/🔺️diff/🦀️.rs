//! 🔺️ Diff constructor for `SetMepElement`: a sparse MEP element patch of exactly the provided fields that differ. The element that results must break none of the create rules. Whole-list ruling: the path
//! is ONE centre line, so a provided path replaces the path as one field. Providing only equal values, or no field, is a no-op.

use super::super::component_rules;
use super::SetMepElement;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetMepElement, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.mep_elements.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("MEP element \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let next = payload.patch().write(record);
    if let Some(fault) = component_rules::mep_fault(base, &next) {
        return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(None));
    }
    let change = payload.patch().minimal(record);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("MEP element \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::mep_elements(payload.id.clone(), Entry::Patched(change)))
}
