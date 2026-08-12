//! 🔺️ Sparse diff construction for the `users` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateUserProfile, DeleteUserProfile, RenameUserProfile, ReplaceUserProfile};
use crate::artifacts::program::diff::{ProgramUsersDelta, ProgramUsersPatchEntry};
use crate::artifacts::program::registers::UserProfilePatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.users` on apply.
pub fn diff_create(payload: &CreateUserProfile, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { users: Some(ProgramUsersDelta { added: vec![payload.user_profile.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteUserProfile, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { users: Some(ProgramUsersDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameUserProfile, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = UserProfilePatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { users: Some(ProgramUsersDelta { patched: vec![ProgramUsersPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceUserProfile, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.users.iter().find(|row| row.header.id == payload.user_profile.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.user_profile).expect("diff_patch always produces a full patch");
    ProgramDiff { users: Some(ProgramUsersDelta { patched: vec![ProgramUsersPatchEntry { id: payload.user_profile.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
