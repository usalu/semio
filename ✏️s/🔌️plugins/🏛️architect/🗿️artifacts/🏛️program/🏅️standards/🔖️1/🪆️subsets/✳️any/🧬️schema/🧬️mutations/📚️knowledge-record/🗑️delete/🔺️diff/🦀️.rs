//! 🔺️ Sparse diff construction for the `delete-knowledge-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📚knowledge` per Wave C.

use super::DeleteKnowledgeRecord;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff); else removes the target
/// row from the working-scene cache and re-mints a fresh content-addressed `table` child handle
/// over the remaining rows.
pub fn diff(payload: &DeleteKnowledgeRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let mut records = crate::program_knowledge(base);
    if !records.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No knowledge record exists with this id.", [payload.id.0.clone()]);
    }
    records.retain(|row| row.header.id != payload.id);
    protocol::MutationOutcome::new(ProgramDiff { knowledge: Some(crate::knowledge_child_from_records(&records)), ..Default::default() })
}
