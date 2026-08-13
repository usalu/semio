//! 🔺️ Sparse diff construction for the `rename-knowledge-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📚knowledge` per Wave C.

use super::mutation::RenameKnowledgeRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Sets the target row's `header.name` within the working-scene cache, then re-mints a fresh
/// content-addressed `table` child handle. Missing target ⇒ the re-minted handle carries unchanged
/// rows (an effective no-op, same observable outcome as the former sparse-patch shape's no-op on
/// an unmatched id).
pub fn diff(payload: &RenameKnowledgeRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let mut records = crate::artifacts::program::program_knowledge(base);
    if let Some(existing) = records.iter_mut().find(|row| row.header.id == payload.id) {
        existing.header.name = payload.new_name.clone();
    }
    ProgramDiff { knowledge: Some(crate::artifacts::program::knowledge_child_from_records(&records)), ..Default::default() }
}
