//! 🔺️ Sparse diff construction for the `rename-knowledge-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📚knowledge` per Wave C.

use super::mutation::RenameKnowledgeRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Sets the target row's `header.name` within the working-scene cache, then re-mints a fresh
/// content-addressed `table` child handle. Error `mutation.target-missing` if absent, Warning
/// `mutation.no-op` if the name is unchanged (both empty diff).
pub async fn diff(payload: &RenameKnowledgeRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let mut records = crate::artifacts::program::program_knowledge(base);
    let Some(existing) = records.iter_mut().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No knowledge record exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This knowledge record already has this name.").at([payload.id.0.clone()])]);
    }
    existing.header.name = payload.new_name.clone();
    protocol::MutationOutcome::new(ProgramDiff { knowledge: Some(crate::artifacts::program::knowledge_child_from_records(&records)), ..Default::default() })
}
