//! ↩️ Inverse (undo) construction for the `replace-priority-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `⭐priorities` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::ReplacePriorityRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.priorities.iter().find(|row| row.header.id == payload.priority_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplacePriorityRecord(super::mutation::ReplacePriorityRecord { priority_record: existing.clone() })],
        None => Vec::new(),
    }
}
