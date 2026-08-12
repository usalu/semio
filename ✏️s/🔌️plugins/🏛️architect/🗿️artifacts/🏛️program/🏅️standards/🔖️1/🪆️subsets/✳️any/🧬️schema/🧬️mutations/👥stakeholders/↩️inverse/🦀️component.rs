//! ↩️ Inverse (undo) construction for the `stakeholders` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateStakeholder, DeleteStakeholder, RenameStakeholder, ReplaceStakeholder};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateStakeholder, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteStakeholder(DeleteStakeholder { id: payload.stakeholder.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteStakeholder, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.stakeholders.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateStakeholder(CreateStakeholder { stakeholder: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameStakeholder, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.stakeholders.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameStakeholder(RenameStakeholder { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceStakeholder, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.stakeholders.iter().find(|row| row.header.id == payload.stakeholder.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceStakeholder(ReplaceStakeholder { stakeholder: existing.clone() })],
        None => Vec::new(),
    }
}
