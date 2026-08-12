//! 🔺️ Sparse diff construction for the `rename-status-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📶status-records` per Wave C.

use super::mutation::RenameStatusRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramStatusRecordsDelta, ProgramStatusRecordsPatchEntry};
use crate::artifacts::program::registers::StatusRecordPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameStatusRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = StatusRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { status_records: Some(ProgramStatusRecordsDelta { patched: vec![ProgramStatusRecordsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
