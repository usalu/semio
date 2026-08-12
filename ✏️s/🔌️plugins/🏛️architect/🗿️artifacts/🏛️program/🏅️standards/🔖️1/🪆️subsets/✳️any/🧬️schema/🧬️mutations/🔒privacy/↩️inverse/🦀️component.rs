//! ↩️ Inverse (undo) construction for the `privacy` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreatePrivacyRequirement, DeletePrivacyRequirement, RenamePrivacyRequirement, ReplacePrivacyRequirement};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreatePrivacyRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeletePrivacyRequirement(DeletePrivacyRequirement { id: payload.privacy_requirement.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeletePrivacyRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.privacy.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreatePrivacyRequirement(CreatePrivacyRequirement { privacy_requirement: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenamePrivacyRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.privacy.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenamePrivacyRequirement(RenamePrivacyRequirement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplacePrivacyRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.privacy.iter().find(|row| row.header.id == payload.privacy_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplacePrivacyRequirement(ReplacePrivacyRequirement { privacy_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
