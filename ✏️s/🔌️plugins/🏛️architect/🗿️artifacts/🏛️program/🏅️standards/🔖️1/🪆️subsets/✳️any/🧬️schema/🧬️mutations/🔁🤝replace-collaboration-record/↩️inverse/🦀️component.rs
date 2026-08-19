//! ↩️ Inverse (undo) construction for the `replace-collaboration-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🤝collaboration` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::ReplaceCollaborationRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.collaboration.iter().find(|row| row.header.id == payload.collaboration_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceCollaborationRecord(super::mutation::ReplaceCollaborationRecord { collaboration_record: existing.clone() })],
        None => Vec::new(),
    }
}
