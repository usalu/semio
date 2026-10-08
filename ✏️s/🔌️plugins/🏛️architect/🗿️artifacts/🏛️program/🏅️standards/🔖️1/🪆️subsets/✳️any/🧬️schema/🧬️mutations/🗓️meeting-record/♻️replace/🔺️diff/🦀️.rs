//! 🔺️ Sparse diff construction for the `replace-meeting-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗓️meetings` per Wave C.

use super::ReplaceMeetingRecord;
use crate::diff::ProgramMeetingsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceMeetingRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.meeting_record.header.id;
    let Some(position) = base.meetings.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No meeting record exists with this id.", [id.0.clone()]);
    };
    if base.meetings[position] == payload.meeting_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This meeting record already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramMeetingsDelta::removal(&base.meetings, position);
    delta.absorb(ProgramMeetingsDelta::insertion(position, payload.meeting_record.clone()));
    protocol::MutationOutcome::new(ProgramDiff { meetings: Some(delta), ..Default::default() })
}
