//! ↩️ Inverse (undo) construction for the `security` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateSecurityRequirement, DeleteSecurityRequirement, RenameSecurityRequirement, ReplaceSecurityRequirement};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateSecurityRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteSecurityRequirement(DeleteSecurityRequirement { id: payload.security_requirement.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteSecurityRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.security.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateSecurityRequirement(CreateSecurityRequirement { security_requirement: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameSecurityRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.security.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameSecurityRequirement(RenameSecurityRequirement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceSecurityRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.security.iter().find(|row| row.header.id == payload.security_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceSecurityRequirement(ReplaceSecurityRequirement { security_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
