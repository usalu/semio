//! 🔺️ Sparse diff construction for the `replace-user-profile` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧑users` per Wave C.

use super::ReplaceUserProfile;
use crate::diff::ProgramUsersDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceUserProfile, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.user_profile.header.id;
    let Some(position) = base.users.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No user profile exists with this id.", [id.0.clone()]);
    };
    if base.users[position] == payload.user_profile {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This user profile already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.users.len()).then(|| base.users.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { users: Some(ProgramUsersDelta { removed: vec![id.0.clone()], added: vec![payload.user_profile.clone()], reordered, ..Default::default() }), ..Default::default() })
}
