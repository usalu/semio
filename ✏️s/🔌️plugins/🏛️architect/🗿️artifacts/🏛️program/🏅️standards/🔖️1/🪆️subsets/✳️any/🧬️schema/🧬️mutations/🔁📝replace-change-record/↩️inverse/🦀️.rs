//! ↩️ Inverse (undo) construction for the `replace-change-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📝changes` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::ReplaceChangeRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.changes.iter().find(|row| row.header.id == payload.change_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceChangeRecord(super::ReplaceChangeRecord { change_record: existing.clone() })],
        None => Vec::new(),
    }
}
