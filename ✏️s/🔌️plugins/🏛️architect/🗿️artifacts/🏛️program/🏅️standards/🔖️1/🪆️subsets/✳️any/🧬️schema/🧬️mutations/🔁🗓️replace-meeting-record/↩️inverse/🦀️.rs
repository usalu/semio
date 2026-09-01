//! ↩️ Inverse (undo) construction for the `replace-meeting-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🗓️meetings` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::ReplaceMeetingRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.meetings.iter().find(|row| row.header.id == payload.meeting_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceMeetingRecord(super::ReplaceMeetingRecord { meeting_record: existing.clone() })],
        None => Vec::new(),
    }
}
