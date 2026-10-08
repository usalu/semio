//! 🔺️ Sparse diff construction for the `create-knowledge-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📚knowledge` per Wave C.

use super::CreateKnowledgeRecord;
use crate::diff::ProgramKnowledgeDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.index-out-of-range` if `index` lies past the end (both empty diff); else `added = [payload row]`, plus `reordered` (the base order with the row inserted at `index`) unless the row lands last.
pub fn diff(payload: &CreateKnowledgeRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.knowledge_record.header.id;
    if base.knowledge_payload.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A knowledge record already exists with this id.", [id.0.clone()]);
    }
    let length = base.knowledge_payload.len();
    let at = payload.index.unwrap_or(length);
    if at > length {
        return protocol::MutationOutcome::error("mutation.index-out-of-range", "The index lies beyond the end of the knowledge record list.", [id.0.clone()]);
    }
    let reordered = (at < length).then(|| {
        let mut order: Vec<String> = base.knowledge_payload.iter().map(|row| row.header.id.0.clone()).collect();
        order.insert(at, id.0.clone());
        order
    });
    protocol::MutationOutcome::new(ProgramDiff { knowledge: Some(ProgramKnowledgeDelta { added: vec![payload.knowledge_record.clone()], reordered, ..Default::default() }), ..Default::default() })
}
