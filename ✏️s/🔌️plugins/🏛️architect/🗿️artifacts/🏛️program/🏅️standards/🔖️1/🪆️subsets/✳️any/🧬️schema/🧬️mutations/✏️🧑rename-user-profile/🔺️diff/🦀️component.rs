//! 🔺️ Sparse diff construction for the `rename-user-profile` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧑users` per Wave C.

use super::mutation::RenameUserProfile;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramUsersDelta, ProgramUsersPatchEntry};
use crate::artifacts::program::registers::UserProfilePatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameUserProfile, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = UserProfilePatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { users: Some(ProgramUsersDelta { patched: vec![ProgramUsersPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
