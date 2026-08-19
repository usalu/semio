//! ↩️ Inverse (undo) construction for the `create-user-profile` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧑users` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::mutation::CreateUserProfile, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteUserProfile(super::super::delete_user_profile::mutation::DeleteUserProfile { id: payload.user_profile.header.id.clone() })]
}
