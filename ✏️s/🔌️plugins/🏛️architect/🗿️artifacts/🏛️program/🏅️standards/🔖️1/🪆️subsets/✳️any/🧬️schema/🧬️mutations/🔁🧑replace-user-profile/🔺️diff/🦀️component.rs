//! 🔺️ Sparse diff construction for the `replace-user-profile` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧑users` per Wave C.

use super::mutation::ReplaceUserProfile;
use protocol::Patchable;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramUsersDelta, ProgramUsersPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceUserProfile, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.users.iter().find(|row| row.header.id == payload.user_profile.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.user_profile).expect("diff_patch always produces a full patch");
    ProgramDiff { users: Some(ProgramUsersDelta { patched: vec![ProgramUsersPatchEntry { id: payload.user_profile.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
