//! 🔺️ Sparse diff construction for the `delete-knowledge-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📚knowledge` per Wave C.

use super::DeleteKnowledgeRecord;
use crate::diff::ProgramKnowledgeDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff); else `removed = [id]` — `apply` re-derives the composed child handle from the remaining rows.
pub fn diff(payload: &DeleteKnowledgeRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.knowledge_payload.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No knowledge record exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { knowledge: Some(ProgramKnowledgeDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
