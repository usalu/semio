//! 🔺️ Sparse diff construction for the `create-meeting-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗓️meetings` per Wave C.

use super::mutation::CreateMeetingRecord;
use crate::artifacts::program::diff::ProgramMeetingsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub fn diff(payload: &CreateMeetingRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.meeting_record.header.id.clone();
    if base.meetings.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A meeting record already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { meetings: Some(ProgramMeetingsDelta { added: vec![payload.meeting_record.clone()], ..Default::default() }), ..Default::default() })
}
