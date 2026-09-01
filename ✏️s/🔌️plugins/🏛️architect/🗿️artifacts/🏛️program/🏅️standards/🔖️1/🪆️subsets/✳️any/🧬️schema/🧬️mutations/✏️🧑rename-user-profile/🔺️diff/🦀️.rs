//! 🔺️ Sparse diff construction for the `rename-user-profile` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧑users` per Wave C.

use super::RenameUserProfile;
use crate::artifacts::program::diff::{ProgramUsersDelta, ProgramUsersPatchEntry};
use crate::artifacts::program::registers::UserProfilePatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenameUserProfile, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.users.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No user profile exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This user profile already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = UserProfilePatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { users: Some(ProgramUsersDelta { patched: vec![ProgramUsersPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
