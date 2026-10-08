//! 🔺️ Sparse diff construction for the `create-knowledge-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📚knowledge` per Wave C.

use super::CreateKnowledgeRecord;
use crate::diff::ProgramKnowledgeDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff); else `added = [payload row]` — `apply` re-derives the composed child handle from the rows.
pub fn diff(payload: &CreateKnowledgeRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.knowledge_record.header.id;
    if base.knowledge_payload.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A knowledge record already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { knowledge: Some(ProgramKnowledgeDelta { added: vec![payload.knowledge_record.clone()], ..Default::default() }), ..Default::default() })
}
