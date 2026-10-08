//! ↕️ `update-through-thickness-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateThroughThicknessInputs;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993SectionEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateThroughThicknessInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let edit = match base.sections.iter().position(|row| row.id == payload.section.id) {
        Some(index) if base.sections[index] == payload.section => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => En1993SectionEdit::replace(index, payload.section.id.clone(), payload.section.clone()),
        None => En1993SectionEdit::insert(base.sections.len(), payload.section.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { sections: vec![edit], ..Default::default() })
}
