//! 🔺️ Sparse diff construction for the `replace-knowledge-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📚knowledge` per Wave C.

use super::mutation::ReplaceKnowledgeRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🔁️ Whole-value swap of one row's non-identity content within the working-scene cache, then
/// re-mint a fresh content-addressed `table` child handle. Target absent from `base` ⇒ empty diff
/// (nothing to change) — same observable behavior as the former sparse-patch shape.
pub fn diff(payload: &ReplaceKnowledgeRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let mut records = crate::artifacts::program::program_knowledge(base);
    let Some(existing) = records.iter_mut().find(|row| row.header.id == payload.knowledge_record.header.id) else {
        return ProgramDiff::default();
    };
    *existing = payload.knowledge_record.clone();
    ProgramDiff { knowledge: Some(crate::artifacts::program::knowledge_child_from_records(&records)), ..Default::default() }
}
