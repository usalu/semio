//! 🔺️ Sparse diff construction for the `create-conflict` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚔️conflicts` per Wave C.

use super::mutation::CreateConflict;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramConflictsDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.conflicts` on apply.
pub fn diff(payload: &CreateConflict, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { conflicts: Some(ProgramConflictsDelta { added: vec![payload.conflict.clone()], ..Default::default() }), ..Default::default() }
}
