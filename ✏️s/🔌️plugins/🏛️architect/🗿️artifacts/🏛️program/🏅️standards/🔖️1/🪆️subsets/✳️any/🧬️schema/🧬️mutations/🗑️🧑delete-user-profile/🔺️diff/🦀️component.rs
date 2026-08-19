//! 🔺️ Sparse diff construction for the `delete-user-profile` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧑users` per Wave C.

use super::mutation::DeleteUserProfile;
use crate::artifacts::program::diff::ProgramUsersDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub async fn diff(payload: &DeleteUserProfile, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.users.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No user profile exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { users: Some(ProgramUsersDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
