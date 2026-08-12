//! 🔺️ Sparse diff construction for the `rename-conflict` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚔️conflicts` per Wave C.

use super::mutation::RenameConflict;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramConflictsDelta, ProgramConflictsPatchEntry};
use crate::artifacts::program::registers::ConflictPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameConflict, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ConflictPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { conflicts: Some(ProgramConflictsDelta { patched: vec![ProgramConflictsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
