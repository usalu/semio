//! ↩️ Inverse (undo) construction for the `workshops` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateWorkshop, DeleteWorkshop, RenameWorkshop, ReplaceWorkshop};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateWorkshop, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteWorkshop(DeleteWorkshop { id: payload.workshop.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteWorkshop, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.workshops.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateWorkshop(CreateWorkshop { workshop: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameWorkshop, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.workshops.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameWorkshop(RenameWorkshop { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceWorkshop, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.workshops.iter().find(|row| row.header.id == payload.workshop.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceWorkshop(ReplaceWorkshop { workshop: existing.clone() })],
        None => Vec::new(),
    }
}
