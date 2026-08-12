//! ↩️ Inverse (undo) construction for the `assumptions` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateAssumption, DeleteAssumption, RenameAssumption, ReplaceAssumption};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateAssumption, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteAssumption(DeleteAssumption { id: payload.assumption.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteAssumption, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.assumptions.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateAssumption(CreateAssumption { assumption: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameAssumption, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.assumptions.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameAssumption(RenameAssumption { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceAssumption, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.assumptions.iter().find(|row| row.header.id == payload.assumption.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceAssumption(ReplaceAssumption { assumption: existing.clone() })],
        None => Vec::new(),
    }
}
