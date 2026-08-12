//! 🔺️ Sparse diff construction for the `create-status-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📶status-records` per Wave C.

use super::mutation::CreateStatusRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramStatusRecordsDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.status_records` on apply.
pub fn diff(payload: &CreateStatusRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { status_records: Some(ProgramStatusRecordsDelta { added: vec![payload.status_record.clone()], ..Default::default() }), ..Default::default() }
}
