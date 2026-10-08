//! 🔺️ Sparse diff construction for the `create-user-profile` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧑users` per Wave C.

use super::CreateUserProfile;
use crate::diff::ProgramUsersDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateUserProfile, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.user_profile.header.id;
    if base.users.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An user profile already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.users.len());
    if at > base.users.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the user profile list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { users: Some(ProgramUsersDelta::insertion(at, payload.user_profile.clone())), ..Default::default() })
}
