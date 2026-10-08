//! 🔺️ Sparse diff construction for the `create-program-element` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧱elements` per Wave C.

use super::CreateProgramElement;
use crate::diff::ProgramElementsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.index-out-of-range` if `index` lies past the end (both empty diff); else `added = [payload row]`, plus `reordered` (the base order with the row inserted at `index`) unless the row lands last.
pub fn diff(payload: &CreateProgramElement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.program_element.header.id;
    if base.elements.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A program element already exists with this id.", [id.0.clone()]);
    }
    let length = base.elements.len();
    let at = payload.index.unwrap_or(length);
    if at > length {
        return protocol::MutationOutcome::error("mutation.index-out-of-range", "The index lies beyond the end of the program element list.", [id.0.clone()]);
    }
    let reordered = (at < length).then(|| {
        let mut order: Vec<String> = base.elements.iter().map(|row| row.header.id.0.clone()).collect();
        order.insert(at, id.0.clone());
        order
    });
    protocol::MutationOutcome::new(ProgramDiff { elements: Some(ProgramElementsDelta { added: vec![payload.program_element.clone()], reordered, ..Default::default() }), ..Default::default() })
}
