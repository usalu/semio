//! 🔺️ Sparse diff construction for the `replace-meeting-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗓️meetings` per Wave C.

use super::ReplaceMeetingRecord;
use crate::diff::ProgramMeetingsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplaceMeetingRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.meeting_record.header.id;
    let Some(position) = base.meetings.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No meeting record exists with this id.", [id.0.clone()]);
    };
    if base.meetings[position] == payload.meeting_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This meeting record already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.meetings.len()).then(|| base.meetings.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { meetings: Some(ProgramMeetingsDelta { removed: vec![id.0.clone()], added: vec![payload.meeting_record.clone()], reordered, ..Default::default() }), ..Default::default() })
}
