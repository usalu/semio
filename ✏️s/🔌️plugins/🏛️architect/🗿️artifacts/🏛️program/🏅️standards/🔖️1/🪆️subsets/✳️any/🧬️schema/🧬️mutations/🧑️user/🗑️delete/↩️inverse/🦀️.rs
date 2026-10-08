//! ↩️ Inverse (undo) construction for the `delete-user-profile` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧑users` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteUserProfile, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.users.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateUserProfile(super::super::create_user_profile::CreateUserProfile { user_profile: base.users[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
