//! ↩️ Inverse (undo) construction for the `meetings` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff.

use super::mutation::{CreateMeetingRecord, DeleteMeetingRecord, RenameMeetingRecord, ReplaceMeetingRecord};
use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse_create(payload: &CreateMeetingRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteMeetingRecord(DeleteMeetingRecord { id: payload.meeting_record.header.id.clone() })]
}

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse_delete(payload: &DeleteMeetingRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.meetings.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateMeetingRecord(CreateMeetingRecord { meeting_record: existing.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse_rename(payload: &RenameMeetingRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.meetings.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameMeetingRecord(RenameMeetingRecord { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse_replace(payload: &ReplaceMeetingRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.meetings.iter().find(|row| row.header.id == payload.meeting_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceMeetingRecord(ReplaceMeetingRecord { meeting_record: existing.clone() })],
        None => Vec::new(),
    }
}
