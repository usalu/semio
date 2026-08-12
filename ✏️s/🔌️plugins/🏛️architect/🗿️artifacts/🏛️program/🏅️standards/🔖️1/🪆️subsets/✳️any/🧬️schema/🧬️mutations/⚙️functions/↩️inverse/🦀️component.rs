//! ↩️ Inverse (undo) construction for the `functions` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateFunction, DeleteFunction, RenameFunction, ReplaceFunction};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateFunction, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteFunction(DeleteFunction { id: payload.function.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteFunction, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.functions.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateFunction(CreateFunction { function: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameFunction, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.functions.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameFunction(RenameFunction { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceFunction, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.functions.iter().find(|row| row.header.id == payload.function.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceFunction(ReplaceFunction { function: existing.clone() })],
        None => Vec::new(),
    }
}
