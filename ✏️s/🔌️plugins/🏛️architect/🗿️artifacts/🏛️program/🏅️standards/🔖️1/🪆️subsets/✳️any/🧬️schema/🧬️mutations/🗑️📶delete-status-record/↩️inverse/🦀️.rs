//! ↩️ Inverse (undo) construction for the `delete-status-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📶status-records` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::DeleteStatusRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.status_records.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateStatusRecord(super::super::create_status_record::CreateStatusRecord { status_record: existing.clone() })],
        None => Vec::new(),
    }
}
