//! 🔺️ Sparse diff construction for the `replace-status-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📶status-records` per Wave C.

use super::mutation::ReplaceStatusRecord;
use crate::artifacts::program::diff::{ProgramStatusRecordsDelta, ProgramStatusRecordsPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceStatusRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.status_records.iter().find(|row| row.header.id == payload.status_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.status_record).expect("diff_patch always produces a full patch");
    ProgramDiff { status_records: Some(ProgramStatusRecordsDelta { patched: vec![ProgramStatusRecordsPatchEntry { id: payload.status_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
