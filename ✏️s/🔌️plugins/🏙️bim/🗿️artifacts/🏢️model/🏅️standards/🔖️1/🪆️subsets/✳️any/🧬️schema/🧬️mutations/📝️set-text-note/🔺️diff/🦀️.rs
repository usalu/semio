//! 🔺️ Diff constructor for `SetTextNote`: a sparse text note patch of exactly the provided fields that differ. The record that results must break none of the create rules: the storey and the style must exist, the position and rotation are finite and the text is not blank.
//! Providing only equal values, or no field, is a no-op.

use super::super::annotating;
use super::SetTextNote;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetTextNote, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.text_notes.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Text note \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let next = payload.patch().write(record);
    if let Some(fault) = annotating::note_fault(base, &next) {
        let path = fault.path(None);
        return MutationOutcome::refuse(fault.code, fault.message, path);
    }
    let change = payload.patch().minimal(record);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Text note \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::text_notes(payload.id.clone(), Entry::Patched(change)))
}
