//! 🔺️ Sparse diff construction for the `replace-meeting-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🗓️meetings` per Wave C.

use super::ReplaceMeetingRecord;
use crate::artifacts::program::diff::{ProgramMeetingsDelta, ProgramMeetingsPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceMeetingRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.meetings.iter().find(|row| row.header.id == payload.meeting_record.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No meeting record exists with this id.", [payload.meeting_record.header.id.0.clone()]);
    };
    if existing == &payload.meeting_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This meeting record already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.meeting_record).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { meetings: Some(ProgramMeetingsDelta { patched: vec![ProgramMeetingsPatchEntry { id: payload.meeting_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
