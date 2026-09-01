//! ↩️ Inverse (undo) construction for the `delete-priority-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `⭐priorities` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::DeletePriorityRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.priorities.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreatePriorityRecord(super::super::create_priority_record::CreatePriorityRecord { priority_record: existing.clone() })],
        None => Vec::new(),
    }
}
