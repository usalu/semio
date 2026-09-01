//! 🔺️ Sparse diff construction for the `delete-knowledge-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📚knowledge` per Wave C.

use super::DeleteKnowledgeRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff); else removes the target
/// row from the working-scene cache and re-mints a fresh content-addressed `table` child handle
/// over the remaining rows.
pub async fn diff(payload: &DeleteKnowledgeRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let mut records = crate::artifacts::program::program_knowledge(base);
    if !records.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No knowledge record exists with this id.", [payload.id.0.clone()]);
    }
    records.retain(|row| row.header.id != payload.id);
    protocol::MutationOutcome::new(ProgramDiff { knowledge: Some(crate::artifacts::program::knowledge_child_from_records(&records)), ..Default::default() })
}
