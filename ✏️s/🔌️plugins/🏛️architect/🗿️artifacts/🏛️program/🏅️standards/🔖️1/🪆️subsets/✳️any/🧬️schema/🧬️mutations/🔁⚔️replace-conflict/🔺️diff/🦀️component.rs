//! 🔺️ Sparse diff construction for the `replace-conflict` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⚔️conflicts` per Wave C.

use super::mutation::ReplaceConflict;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramConflictsDelta, ProgramConflictsPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceConflict, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.conflicts.iter().find(|row| row.header.id == payload.conflict.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.conflict).expect("diff_patch always produces a full patch");
    ProgramDiff { conflicts: Some(ProgramConflictsDelta { patched: vec![ProgramConflictsPatchEntry { id: payload.conflict.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
