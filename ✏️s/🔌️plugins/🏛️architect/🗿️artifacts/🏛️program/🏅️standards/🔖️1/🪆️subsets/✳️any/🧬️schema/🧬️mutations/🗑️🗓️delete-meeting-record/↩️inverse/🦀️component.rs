//! ↩️ Inverse (undo) construction for the `delete-meeting-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🗓️meetings` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::DeleteMeetingRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.meetings.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateMeetingRecord(super::super::create_meeting_record::mutation::CreateMeetingRecord { meeting_record: existing.clone() })],
        None => Vec::new(),
    }
}
