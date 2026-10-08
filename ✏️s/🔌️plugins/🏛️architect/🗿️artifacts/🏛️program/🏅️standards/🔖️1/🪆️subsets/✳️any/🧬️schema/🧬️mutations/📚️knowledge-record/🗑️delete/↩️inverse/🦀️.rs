//! ↩️ Inverse (undo) construction for the `delete-knowledge-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📚knowledge` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteKnowledgeRecord, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.knowledge_payload.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateKnowledgeRecord(super::super::create_knowledge_record::CreateKnowledgeRecord { knowledge_record: base.knowledge_payload[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
