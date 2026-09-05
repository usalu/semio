//! ↩️ Inverse (undo) construction for the `delete-knowledge-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📚knowledge` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::DeleteKnowledgeRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    let records = crate::artifacts::program::program_knowledge(base);
    match records.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateKnowledgeRecord(super::super::create_knowledge_record::CreateKnowledgeRecord { knowledge_record: existing.clone() })],
        None => Vec::new(),
    }
}
