//! ↩️ Inverse (undo) construction for the `delete-meeting-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🗓️meetings` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteMeetingRecord, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.meetings.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateMeetingRecord(super::super::create_meeting_record::CreateMeetingRecord { meeting_record: base.meetings[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
