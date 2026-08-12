//! ↩️ Inverse (undo) construction for the `infrastructure` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateInfrastructureRequirement, DeleteInfrastructureRequirement, RenameInfrastructureRequirement, ReplaceInfrastructureRequirement};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateInfrastructureRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteInfrastructureRequirement(DeleteInfrastructureRequirement { id: payload.infrastructure_requirement.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteInfrastructureRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.infrastructure.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateInfrastructureRequirement(CreateInfrastructureRequirement { infrastructure_requirement: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameInfrastructureRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.infrastructure.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameInfrastructureRequirement(RenameInfrastructureRequirement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceInfrastructureRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.infrastructure.iter().find(|row| row.header.id == payload.infrastructure_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceInfrastructureRequirement(ReplaceInfrastructureRequirement { infrastructure_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
