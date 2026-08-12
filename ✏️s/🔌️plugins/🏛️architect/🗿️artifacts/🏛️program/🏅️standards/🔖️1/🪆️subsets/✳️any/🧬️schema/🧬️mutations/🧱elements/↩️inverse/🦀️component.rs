//! ↩️ Inverse (undo) construction for the `elements` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateProgramElement, DeleteProgramElement, RenameProgramElement, ReplaceProgramElement};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateProgramElement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteProgramElement(DeleteProgramElement { id: payload.program_element.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteProgramElement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.elements.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateProgramElement(CreateProgramElement { program_element: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameProgramElement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.elements.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameProgramElement(RenameProgramElement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceProgramElement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.elements.iter().find(|row| row.header.id == payload.program_element.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceProgramElement(ReplaceProgramElement { program_element: existing.clone() })],
        None => Vec::new(),
    }
}
