//! ↩️ Inverse (undo) construction for the `requirements` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateRequirement, DeleteRequirement, RenameRequirement, ReplaceRequirement};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteRequirement(DeleteRequirement { id: payload.requirement.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.requirements.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateRequirement(CreateRequirement { requirement: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.requirements.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameRequirement(RenameRequirement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.requirements.iter().find(|row| row.header.id == payload.requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceRequirement(ReplaceRequirement { requirement: existing.clone() })],
        None => Vec::new(),
    }
}
