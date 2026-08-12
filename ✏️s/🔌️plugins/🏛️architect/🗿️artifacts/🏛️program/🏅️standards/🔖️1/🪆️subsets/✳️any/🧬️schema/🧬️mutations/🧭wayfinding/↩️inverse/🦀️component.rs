//! ↩️ Inverse (undo) construction for the `wayfinding` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateWayfindingRequirement, DeleteWayfindingRequirement, RenameWayfindingRequirement, ReplaceWayfindingRequirement};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateWayfindingRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteWayfindingRequirement(DeleteWayfindingRequirement { id: payload.wayfinding_requirement.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteWayfindingRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.wayfinding.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateWayfindingRequirement(CreateWayfindingRequirement { wayfinding_requirement: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameWayfindingRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.wayfinding.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameWayfindingRequirement(RenameWayfindingRequirement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceWayfindingRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.wayfinding.iter().find(|row| row.header.id == payload.wayfinding_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceWayfindingRequirement(ReplaceWayfindingRequirement { wayfinding_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
