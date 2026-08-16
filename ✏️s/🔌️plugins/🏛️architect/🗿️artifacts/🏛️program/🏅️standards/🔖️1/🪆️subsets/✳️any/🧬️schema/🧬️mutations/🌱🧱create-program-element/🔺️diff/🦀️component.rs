//! 🔺️ Sparse diff construction for the `create-program-element` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧱elements` per Wave C.

use super::mutation::CreateProgramElement;
use crate::artifacts::program::diff::ProgramElementsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateProgramElement, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.program_element.header.id.clone();
    if base.elements.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A program element already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { elements: Some(ProgramElementsDelta { added: vec![payload.program_element.clone()], ..Default::default() }), ..Default::default() })
}
