//! ↩️ Inverse (undo) construction for the `replace-status-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📶status-records` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::ReplaceStatusRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.status_records.iter().find(|row| row.header.id == payload.status_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceStatusRecord(super::mutation::ReplaceStatusRecord { status_record: existing.clone() })],
        None => Vec::new(),
    }
}
