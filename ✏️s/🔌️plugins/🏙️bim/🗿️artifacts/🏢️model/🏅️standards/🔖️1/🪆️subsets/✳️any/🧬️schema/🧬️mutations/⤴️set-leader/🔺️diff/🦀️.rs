//! 🔺️ Diff constructor for `SetLeader`: a sparse leader patch of exactly the provided fields that differ. The record that results must break none of the create rules: the storey and the style must exist, the anchor names an existing element or a finite point, the offset is finite and the text is not blank.
//! Providing only equal values, or no field, is a no-op.

use super::super::annotating;
use super::SetLeader;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetLeader, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.leaders.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Leader \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let next = payload.patch().write(record);
    if let Some(fault) = annotating::leader_fault(base, &next) {
        let path = fault.path(None);
        return MutationOutcome::refuse(fault.code, fault.message, path);
    }
    let change = payload.patch().minimal(record);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Leader \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::leaders(payload.id.clone(), Entry::Patched(change)))
}
