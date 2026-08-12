//! ↩️ Inverse (undo) construction for the `resilience` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateResilienceRequirement, DeleteResilienceRequirement, RenameResilienceRequirement, ReplaceResilienceRequirement};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateResilienceRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteResilienceRequirement(DeleteResilienceRequirement { id: payload.resilience_requirement.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteResilienceRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.resilience.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateResilienceRequirement(CreateResilienceRequirement { resilience_requirement: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameResilienceRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.resilience.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameResilienceRequirement(RenameResilienceRequirement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceResilienceRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.resilience.iter().find(|row| row.header.id == payload.resilience_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceResilienceRequirement(ReplaceResilienceRequirement { resilience_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
