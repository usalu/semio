//! 🔺️ Sparse diff construction for the `replace-change-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📝changes` per Wave C.

use super::mutation::ReplaceChangeRecord;
use protocol::Patchable;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramChangesDelta, ProgramChangesPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceChangeRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.changes.iter().find(|row| row.header.id == payload.change_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.change_record).expect("diff_patch always produces a full patch");
    ProgramDiff { changes: Some(ProgramChangesDelta { patched: vec![ProgramChangesPatchEntry { id: payload.change_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
