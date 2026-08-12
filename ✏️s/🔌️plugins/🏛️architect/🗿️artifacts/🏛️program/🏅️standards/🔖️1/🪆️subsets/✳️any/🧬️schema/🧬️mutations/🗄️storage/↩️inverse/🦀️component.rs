//! ↩️ Inverse (undo) construction for the `storage` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateStorageRequirement, DeleteStorageRequirement, RenameStorageRequirement, ReplaceStorageRequirement};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateStorageRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteStorageRequirement(DeleteStorageRequirement { id: payload.storage_requirement.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteStorageRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.storage.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateStorageRequirement(CreateStorageRequirement { storage_requirement: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameStorageRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.storage.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameStorageRequirement(RenameStorageRequirement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceStorageRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.storage.iter().find(|row| row.header.id == payload.storage_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceStorageRequirement(ReplaceStorageRequirement { storage_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
