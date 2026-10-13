//! 🔺️ Sparse diff construction for the `create-program-element` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧱elements` per Wave C.

use super::CreateProgramElement;
use crate::standards::v1::subsets::any::schema::diff::ProgramElementsDelta;
use crate::standards::v1::subsets::any::schema::diff::ProgramDiff;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateProgramElement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.program_element.header.id;
    if base.elements.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A program element already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.elements.len());
    if at > base.elements.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the program element list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { elements: Some(ProgramElementsDelta::insertion(at, payload.program_element.clone())), ..Default::default() })
}
