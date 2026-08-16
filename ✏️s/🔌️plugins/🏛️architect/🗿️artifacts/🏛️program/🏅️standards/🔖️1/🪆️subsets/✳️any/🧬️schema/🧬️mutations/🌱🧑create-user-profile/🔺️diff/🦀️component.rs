//! 🔺️ Sparse diff construction for the `create-user-profile` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧑users` per Wave C.

use super::mutation::CreateUserProfile;
use crate::artifacts::program::diff::ProgramUsersDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateUserProfile, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.user_profile.header.id.clone();
    if base.users.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An user profile already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { users: Some(ProgramUsersDelta { added: vec![payload.user_profile.clone()], ..Default::default() }), ..Default::default() })
}
