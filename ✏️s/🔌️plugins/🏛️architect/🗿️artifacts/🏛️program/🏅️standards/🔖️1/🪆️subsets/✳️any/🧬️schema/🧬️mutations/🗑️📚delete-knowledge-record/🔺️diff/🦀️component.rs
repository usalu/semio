//! 🔺️ Sparse diff construction for the `delete-knowledge-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📚knowledge` per Wave C.

use super::mutation::DeleteKnowledgeRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Removes the target row from the working-scene cache, then re-mints a fresh
/// content-addressed `table` child handle over the remaining rows.
pub fn diff(payload: &DeleteKnowledgeRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let mut records = crate::artifacts::program::program_knowledge(base);
    records.retain(|row| row.header.id != payload.id);
    ProgramDiff { knowledge: Some(crate::artifacts::program::knowledge_child_from_records(&records)), ..Default::default() }
}
