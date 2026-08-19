//! ↩️ Inverse (undo) construction for the `create-meeting-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🗓️meetings` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::mutation::CreateMeetingRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteMeetingRecord(super::super::delete_meeting_record::mutation::DeleteMeetingRecord { id: payload.meeting_record.header.id.clone() })]
}
