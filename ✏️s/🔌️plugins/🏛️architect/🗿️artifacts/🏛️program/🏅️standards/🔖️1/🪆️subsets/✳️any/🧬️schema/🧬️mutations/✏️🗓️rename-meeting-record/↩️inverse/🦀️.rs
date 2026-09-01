//! ↩️ Inverse (undo) construction for the `rename-meeting-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🗓️meetings` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::RenameMeetingRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.meetings.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameMeetingRecord(super::RenameMeetingRecord { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}
