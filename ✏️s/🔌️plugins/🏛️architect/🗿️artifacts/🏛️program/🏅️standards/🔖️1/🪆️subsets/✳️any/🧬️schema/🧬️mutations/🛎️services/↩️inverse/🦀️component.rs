//! ↩️ Inverse (undo) construction for the `services` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateServiceRequirement, DeleteServiceRequirement, RenameServiceRequirement, ReplaceServiceRequirement};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateServiceRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteServiceRequirement(DeleteServiceRequirement { id: payload.service_requirement.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteServiceRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.services.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateServiceRequirement(CreateServiceRequirement { service_requirement: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameServiceRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.services.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameServiceRequirement(RenameServiceRequirement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceServiceRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.services.iter().find(|row| row.header.id == payload.service_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceServiceRequirement(ReplaceServiceRequirement { service_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
