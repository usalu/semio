//! 🔺️ Sparse diff construction for the `create-knowledge-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📚knowledge` per Wave C.

use super::CreateKnowledgeRecord;
use crate::diff::ProgramKnowledgeDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateKnowledgeRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.knowledge_record.header.id;
    if base.knowledge_payload.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A knowledge record already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.knowledge_payload.len());
    if at > base.knowledge_payload.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the knowledge record list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { knowledge: Some(ProgramKnowledgeDelta::insertion(at, payload.knowledge_record.clone())), ..Default::default() })
}
