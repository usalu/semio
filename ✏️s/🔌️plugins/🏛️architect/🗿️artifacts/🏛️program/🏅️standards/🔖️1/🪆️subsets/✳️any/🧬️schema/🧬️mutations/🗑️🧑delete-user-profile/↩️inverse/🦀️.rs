//! ↩️ Inverse (undo) construction for the `delete-user-profile` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧑users` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::DeleteUserProfile, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.users.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateUserProfile(super::super::create_user_profile::CreateUserProfile { user_profile: existing.clone() })],
        None => Vec::new(),
    }
}
