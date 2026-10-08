//! 🔺️ Sparse diff construction for the `create-meeting-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗓️meetings` per Wave C.

use super::CreateMeetingRecord;
use crate::diff::ProgramMeetingsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateMeetingRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.meeting_record.header.id;
    if base.meetings.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A meeting record already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.meetings.len());
    if at > base.meetings.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the meeting record list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { meetings: Some(ProgramMeetingsDelta::insertion(at, payload.meeting_record.clone())), ..Default::default() })
}
