//! 🔺️ Sparse diff construction for the `rename-meeting-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗓️meetings` per Wave C.

use super::mutation::RenameMeetingRecord;
use crate::artifacts::program::diff::{ProgramMeetingsDelta, ProgramMeetingsPatchEntry};
use crate::artifacts::program::registers::MeetingRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameMeetingRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = MeetingRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { meetings: Some(ProgramMeetingsDelta { patched: vec![ProgramMeetingsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
