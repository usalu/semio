//! 🔺️ Sparse diff construction for the `delete-user-profile` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧑users` per Wave C.

use super::mutation::DeleteUserProfile;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramUsersDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteUserProfile, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { users: Some(ProgramUsersDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
