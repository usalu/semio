//! 🔺️ Sparse diff construction for the `delete-knowledge-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📚knowledge` per Wave C.

use super::DeleteKnowledgeRecord;
use crate::standards::v1::subsets::any::schema::diff::ProgramKnowledgeDelta;
use crate::standards::v1::subsets::any::schema::diff::ProgramDiff;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff); else `removed = [{id, index}]` — `apply` re-derives the composed child handle from the remaining rows.
pub fn diff(payload: &DeleteKnowledgeRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(position) = base.knowledge_payload.iter().position(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No knowledge record exists with this id.", [payload.id.0.clone()]);
    };
    protocol::MutationOutcome::new(ProgramDiff { knowledge: Some(ProgramKnowledgeDelta::removal(&base.knowledge_payload, position)), ..Default::default() })
}
