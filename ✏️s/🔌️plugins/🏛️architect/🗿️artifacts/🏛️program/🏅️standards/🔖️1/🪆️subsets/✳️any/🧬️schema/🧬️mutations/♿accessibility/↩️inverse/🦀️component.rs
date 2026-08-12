//! ↩️ Inverse (undo) construction for the `accessibility` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateAccessibilityRequirement, DeleteAccessibilityRequirement, RenameAccessibilityRequirement, ReplaceAccessibilityRequirement};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateAccessibilityRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteAccessibilityRequirement(DeleteAccessibilityRequirement { id: payload.accessibility_requirement.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteAccessibilityRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.accessibility.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateAccessibilityRequirement(CreateAccessibilityRequirement { accessibility_requirement: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameAccessibilityRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.accessibility.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameAccessibilityRequirement(RenameAccessibilityRequirement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceAccessibilityRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.accessibility.iter().find(|row| row.header.id == payload.accessibility_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceAccessibilityRequirement(ReplaceAccessibilityRequirement { accessibility_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
