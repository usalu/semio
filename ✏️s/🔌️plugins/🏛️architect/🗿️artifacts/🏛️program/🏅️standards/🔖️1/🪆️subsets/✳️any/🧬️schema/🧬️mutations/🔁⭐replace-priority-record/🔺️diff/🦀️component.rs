//! 🔺️ Sparse diff construction for the `replace-priority-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⭐priorities` per Wave C.

use super::mutation::ReplacePriorityRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramPrioritiesDelta, ProgramPrioritiesPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplacePriorityRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.priorities.iter().find(|row| row.header.id == payload.priority_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.priority_record).expect("diff_patch always produces a full patch");
    ProgramDiff { priorities: Some(ProgramPrioritiesDelta { patched: vec![ProgramPrioritiesPatchEntry { id: payload.priority_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
