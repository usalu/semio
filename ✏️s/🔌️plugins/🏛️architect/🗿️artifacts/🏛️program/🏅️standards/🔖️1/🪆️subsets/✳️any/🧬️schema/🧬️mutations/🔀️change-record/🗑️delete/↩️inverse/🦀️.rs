//! ↩️ Inverse (undo) construction for the `delete-change-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📝changes` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::DeleteChangeRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.changes.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateChangeRecord(super::super::create_change_record::CreateChangeRecord { change_record: existing.clone() })],
        None => Vec::new(),
    }
}
