//! 🔺️ Sparse diff construction for the `create-user-profile` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧑users` per Wave C.

use super::CreateUserProfile;
use crate::diff::ProgramUsersDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.index-out-of-range` if `index` lies past the end (both empty diff); else `added = [payload row]`, plus `reordered` (the base order with the row inserted at `index`) unless the row lands last.
pub fn diff(payload: &CreateUserProfile, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.user_profile.header.id;
    if base.users.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "An user profile already exists with this id.", [id.0.clone()]);
    }
    let length = base.users.len();
    let at = payload.index.unwrap_or(length);
    if at > length {
        return protocol::MutationOutcome::error("mutation.index-out-of-range", "The index lies beyond the end of the user profile list.", [id.0.clone()]);
    }
    let reordered = (at < length).then(|| {
        let mut order: Vec<String> = base.users.iter().map(|row| row.header.id.0.clone()).collect();
        order.insert(at, id.0.clone());
        order
    });
    protocol::MutationOutcome::new(ProgramDiff { users: Some(ProgramUsersDelta { added: vec![payload.user_profile.clone()], reordered, ..Default::default() }), ..Default::default() })
}
