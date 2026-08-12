//! ↩️ Inverse (undo) construction for the `users` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateUserProfile, DeleteUserProfile, RenameUserProfile, ReplaceUserProfile};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateUserProfile, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteUserProfile(DeleteUserProfile { id: payload.user_profile.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteUserProfile, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.users.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateUserProfile(CreateUserProfile { user_profile: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameUserProfile, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.users.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameUserProfile(RenameUserProfile { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceUserProfile, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.users.iter().find(|row| row.header.id == payload.user_profile.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceUserProfile(ReplaceUserProfile { user_profile: existing.clone() })],
        None => Vec::new(),
    }
}
