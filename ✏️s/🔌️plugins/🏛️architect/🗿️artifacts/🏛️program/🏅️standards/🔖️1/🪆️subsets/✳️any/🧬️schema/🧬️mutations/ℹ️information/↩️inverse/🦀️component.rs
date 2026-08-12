//! ↩️ Inverse (undo) construction for the `information` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateInformationRequirement, DeleteInformationRequirement, RenameInformationRequirement, ReplaceInformationRequirement};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateInformationRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteInformationRequirement(DeleteInformationRequirement { id: payload.information_requirement.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteInformationRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.information.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateInformationRequirement(CreateInformationRequirement { information_requirement: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameInformationRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.information.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameInformationRequirement(RenameInformationRequirement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceInformationRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.information.iter().find(|row| row.header.id == payload.information_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceInformationRequirement(ReplaceInformationRequirement { information_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
