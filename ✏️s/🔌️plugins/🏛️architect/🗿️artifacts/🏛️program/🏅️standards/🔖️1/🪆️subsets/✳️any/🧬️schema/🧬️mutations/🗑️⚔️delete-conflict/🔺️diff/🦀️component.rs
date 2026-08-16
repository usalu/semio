//! 🔺️ Sparse diff construction for the `delete-conflict` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚔️conflicts` per Wave C.

use super::mutation::DeleteConflict;
use crate::artifacts::program::diff::ProgramConflictsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteConflict, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { conflicts: Some(ProgramConflictsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
