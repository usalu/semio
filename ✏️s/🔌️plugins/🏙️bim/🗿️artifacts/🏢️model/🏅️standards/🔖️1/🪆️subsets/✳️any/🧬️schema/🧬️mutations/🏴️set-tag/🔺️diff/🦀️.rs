//! 🔺️ Diff constructor for `SetTag`: a sparse tag patch of exactly the provided fields that differ. The record that results must break none of the create rules: the storey and the style must exist, the element is one that can be tagged and the offset is finite.
//! Providing only equal values, or no field, is a no-op.

use super::super::annotating;
use super::SetTag;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetTag, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.tags.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Tag \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let next = payload.patch().write(record);
    if let Some(fault) = annotating::tag_fault(base, &next) {
        let path = fault.path(None);
        return MutationOutcome::refuse(fault.code, fault.message, path);
    }
    let change = payload.patch().minimal(record);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Tag \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::tags(payload.id.clone(), Entry::Patched(change)))
}
