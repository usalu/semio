//! ↩️ Inverse (undo) construction for the `replace-knowledge-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📚knowledge` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::ReplaceKnowledgeRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    let records = crate::program_knowledge(base);
    match records.iter().find(|row| row.header.id == payload.knowledge_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceKnowledgeRecord(super::ReplaceKnowledgeRecord { knowledge_record: existing.clone() })],
        None => Vec::new(),
    }
}
