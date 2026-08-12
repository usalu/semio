//! ↩️ Inverse (undo) construction for the `organizational` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateOrganizationalRequirement, DeleteOrganizationalRequirement, RenameOrganizationalRequirement, ReplaceOrganizationalRequirement};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateOrganizationalRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteOrganizationalRequirement(DeleteOrganizationalRequirement { id: payload.organizational_requirement.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteOrganizationalRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.organizational.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateOrganizationalRequirement(CreateOrganizationalRequirement { organizational_requirement: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameOrganizationalRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.organizational.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameOrganizationalRequirement(RenameOrganizationalRequirement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceOrganizationalRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.organizational.iter().find(|row| row.header.id == payload.organizational_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceOrganizationalRequirement(ReplaceOrganizationalRequirement { organizational_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
