//! 🔺️ Sparse diff construction for the `create-user-profile` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧑users` per Wave C.

use super::mutation::CreateUserProfile;
use crate::artifacts::program::diff::ProgramUsersDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.users` on apply.
pub fn diff(payload: &CreateUserProfile, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { users: Some(ProgramUsersDelta { added: vec![payload.user_profile.clone()], ..Default::default() }), ..Default::default() }
}
