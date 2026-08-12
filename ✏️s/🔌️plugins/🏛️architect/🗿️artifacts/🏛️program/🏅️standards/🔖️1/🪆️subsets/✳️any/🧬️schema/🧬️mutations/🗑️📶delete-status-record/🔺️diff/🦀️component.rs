//! 🔺️ Sparse diff construction for the `delete-status-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📶status-records` per Wave C.

use super::mutation::DeleteStatusRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramStatusRecordsDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteStatusRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { status_records: Some(ProgramStatusRecordsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
