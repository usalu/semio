//! 🔺️ Sparse diff construction for the `delete-meeting-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗓️meetings` per Wave C.

use super::mutation::DeleteMeetingRecord;
use crate::artifacts::program::diff::ProgramMeetingsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteMeetingRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { meetings: Some(ProgramMeetingsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
