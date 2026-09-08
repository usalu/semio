//! ↩️ Inverse (undo) construction for the `delete-program-element` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧱elements` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteProgramElement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.elements.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateProgramElement(super::super::create_program_element::CreateProgramElement { program_element: existing.clone() })],
        None => Vec::new(),
    }
}
