//! 🔺️ Diff constructor for `CreateTextNote`: one created text note entry. The id is free in every collection. The storey and the style must exist, the position and rotation are finite and the text is not blank.
//! Everything the text note shows is inferred from the current geometry of what it names, never stored.

use super::super::annotating;
use super::super::elements;
use super::CreateTextNote;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateTextNote, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(fault) = annotating::note_fault(base, &payload.text_note) {
        let path = fault.path(Some("text_note"));
        return MutationOutcome::refuse(fault.code, fault.message, path);
    }
    MutationOutcome::new(ModelDiff::text_notes(payload.id.clone(), Entry::Created(payload.text_note.clone())))
}
