//! ➕️ `insert-section` diff — inserts the row at its position, clamped to the end of the collection.

use super::InsertSection;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993SectionEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &InsertSection, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.sections.iter().any(|existing| existing.id == payload.section.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Section id {} already exists.", payload.section.id), [payload.section.id.clone()]);
    }
    let index = payload.index.min(base.sections.len());
    protocol::MutationOutcome::new(En1993Diff { sections: vec![En1993SectionEdit::insert(index, payload.section.clone())], ..Default::default() })
}
