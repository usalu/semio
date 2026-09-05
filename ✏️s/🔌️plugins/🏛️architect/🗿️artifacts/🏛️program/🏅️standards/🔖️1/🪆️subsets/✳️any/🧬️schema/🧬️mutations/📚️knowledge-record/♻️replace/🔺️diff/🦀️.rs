//! 🔺️ Sparse diff construction for the `replace-knowledge-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📚knowledge` per Wave C.

use super::ReplaceKnowledgeRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🔁️ Whole-value swap of one row's non-identity content within the working-scene cache, then
/// re-mint a fresh content-addressed `table` child handle. Error `mutation.target-missing` if
/// absent, Warning `mutation.no-op` if the value is unchanged (both empty diff).
pub async fn diff(payload: &ReplaceKnowledgeRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let mut records = crate::artifacts::program::program_knowledge(base);
    let Some(existing) = records.iter_mut().find(|row| row.header.id == payload.knowledge_record.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No knowledge record exists with this id.", [payload.knowledge_record.header.id.0.clone()]);
    };
    if *existing == payload.knowledge_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This knowledge record already matches the requested value.").at([payload.knowledge_record.header.id.0.clone()])]);
    }
    *existing = payload.knowledge_record.clone();
    protocol::MutationOutcome::new(ProgramDiff { knowledge: Some(crate::artifacts::program::knowledge_child_from_records(&records)), ..Default::default() })
}
