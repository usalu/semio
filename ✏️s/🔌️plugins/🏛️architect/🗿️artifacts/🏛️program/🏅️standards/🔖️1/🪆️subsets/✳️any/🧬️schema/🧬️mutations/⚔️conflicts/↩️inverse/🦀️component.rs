//! ↩️ Inverse (undo) construction for the `conflicts` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateConflict, DeleteConflict, RenameConflict, ReplaceConflict};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateConflict, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteConflict(DeleteConflict { id: payload.conflict.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteConflict, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.conflicts.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateConflict(CreateConflict { conflict: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameConflict, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.conflicts.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameConflict(RenameConflict { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceConflict, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.conflicts.iter().find(|row| row.header.id == payload.conflict.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceConflict(ReplaceConflict { conflict: existing.clone() })],
        None => Vec::new(),
    }
}
