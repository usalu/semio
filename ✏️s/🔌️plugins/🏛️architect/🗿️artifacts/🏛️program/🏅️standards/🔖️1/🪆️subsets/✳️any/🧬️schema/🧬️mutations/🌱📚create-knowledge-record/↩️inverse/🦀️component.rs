//! ↩️ Inverse (undo) construction for the `create-knowledge-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📚knowledge` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::mutation::CreateKnowledgeRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteKnowledgeRecord(super::super::delete_knowledge_record::mutation::DeleteKnowledgeRecord { id: payload.knowledge_record.header.id.clone() })]
}
