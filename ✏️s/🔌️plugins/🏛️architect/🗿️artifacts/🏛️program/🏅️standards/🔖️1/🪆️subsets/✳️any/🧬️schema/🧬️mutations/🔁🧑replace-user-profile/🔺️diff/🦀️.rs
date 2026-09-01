//! 🔺️ Sparse diff construction for the `replace-user-profile` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧑users` per Wave C.

use super::ReplaceUserProfile;
use crate::artifacts::program::diff::{ProgramUsersDelta, ProgramUsersPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceUserProfile, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.users.iter().find(|row| row.header.id == payload.user_profile.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No user profile exists with this id.", [payload.user_profile.header.id.0.clone()]);
    };
    if existing == &payload.user_profile {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This user profile already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.user_profile).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { users: Some(ProgramUsersDelta { patched: vec![ProgramUsersPatchEntry { id: payload.user_profile.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
