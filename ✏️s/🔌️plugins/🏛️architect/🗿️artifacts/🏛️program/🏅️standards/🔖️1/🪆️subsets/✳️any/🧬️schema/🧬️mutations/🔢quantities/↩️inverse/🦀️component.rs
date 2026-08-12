//! ↩️ Inverse (undo) construction for the `quantities` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateQuantityRequirement, DeleteQuantityRequirement, RenameQuantityRequirement, ReplaceQuantityRequirement};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateQuantityRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteQuantityRequirement(DeleteQuantityRequirement { id: payload.quantity_requirement.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteQuantityRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.quantities.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateQuantityRequirement(CreateQuantityRequirement { quantity_requirement: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameQuantityRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.quantities.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameQuantityRequirement(RenameQuantityRequirement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceQuantityRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.quantities.iter().find(|row| row.header.id == payload.quantity_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceQuantityRequirement(ReplaceQuantityRequirement { quantity_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
