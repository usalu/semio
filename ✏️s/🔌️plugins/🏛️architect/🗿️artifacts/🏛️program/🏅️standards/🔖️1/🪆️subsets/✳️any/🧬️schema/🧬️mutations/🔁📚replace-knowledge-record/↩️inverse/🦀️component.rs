//! ↩️ Inverse (undo) construction for the `replace-knowledge-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📚knowledge` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::ReplaceKnowledgeRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    let records = crate::artifacts::program::program_knowledge(base);
    match records.iter().find(|row| row.header.id == payload.knowledge_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceKnowledgeRecord(super::mutation::ReplaceKnowledgeRecord { knowledge_record: existing.clone() })],
        None => Vec::new(),
    }
}
