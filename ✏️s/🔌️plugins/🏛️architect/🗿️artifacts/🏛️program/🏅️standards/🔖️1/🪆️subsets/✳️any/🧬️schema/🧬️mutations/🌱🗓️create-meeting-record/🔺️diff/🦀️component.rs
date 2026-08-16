//! 🔺️ Sparse diff construction for the `create-meeting-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗓️meetings` per Wave C.

use super::mutation::CreateMeetingRecord;
use crate::artifacts::program::diff::ProgramMeetingsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.meetings` on apply.
pub fn diff(payload: &CreateMeetingRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { meetings: Some(ProgramMeetingsDelta { added: vec![payload.meeting_record.clone()], ..Default::default() }), ..Default::default() }
}
