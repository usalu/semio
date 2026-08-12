//! ↩️ Inverse (undo) construction for the `communication` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateCommunicationRequirement, DeleteCommunicationRequirement, RenameCommunicationRequirement, ReplaceCommunicationRequirement};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateCommunicationRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteCommunicationRequirement(DeleteCommunicationRequirement { id: payload.communication_requirement.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteCommunicationRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.communication.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateCommunicationRequirement(CreateCommunicationRequirement { communication_requirement: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameCommunicationRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.communication.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameCommunicationRequirement(RenameCommunicationRequirement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceCommunicationRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.communication.iter().find(|row| row.header.id == payload.communication_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceCommunicationRequirement(ReplaceCommunicationRequirement { communication_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
