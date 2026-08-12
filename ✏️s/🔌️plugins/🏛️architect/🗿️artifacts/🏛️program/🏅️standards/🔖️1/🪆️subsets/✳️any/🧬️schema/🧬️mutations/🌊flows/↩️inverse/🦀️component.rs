//! ↩️ Inverse (undo) construction for the `flows` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateFlowRequirement, DeleteFlowRequirement, RenameFlowRequirement, ReplaceFlowRequirement};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateFlowRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteFlowRequirement(DeleteFlowRequirement { id: payload.flow_requirement.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteFlowRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.flows.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateFlowRequirement(CreateFlowRequirement { flow_requirement: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameFlowRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.flows.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameFlowRequirement(RenameFlowRequirement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceFlowRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.flows.iter().find(|row| row.header.id == payload.flow_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceFlowRequirement(ReplaceFlowRequirement { flow_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
