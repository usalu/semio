//! 🔺️ Diff constructor for `SetDimension`: a sparse dimension patch of exactly the provided fields that differ. The record that results must break none of the create rules: the storey and the style must exist, there are at least two anchors that name existing elements or finite points, the direction and offset are finite and a lock is a positive length.
//! Providing only equal values, or no field, is a no-op.

use super::super::annotating;
use super::SetDimension;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetDimension, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.dimensions.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Dimension \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let next = payload.patch().write(record);
    if let Some(fault) = annotating::dimension_fault(base, &next) {
        let path = fault.path(None);
        return MutationOutcome::refuse(fault.code, fault.message, path);
    }
    let change = payload.patch().minimal(record);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Dimension \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::dimensions(payload.id.clone(), Entry::Patched(change)))
}
