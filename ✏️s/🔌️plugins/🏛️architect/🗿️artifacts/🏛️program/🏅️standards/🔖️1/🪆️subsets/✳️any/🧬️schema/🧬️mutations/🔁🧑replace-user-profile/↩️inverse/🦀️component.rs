//! ↩️ Inverse (undo) construction for the `replace-user-profile` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧑users` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::ReplaceUserProfile, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.users.iter().find(|row| row.header.id == payload.user_profile.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceUserProfile(super::mutation::ReplaceUserProfile { user_profile: existing.clone() })],
        None => Vec::new(),
    }
}
