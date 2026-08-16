//! 🔺️ Sparse diff construction for the `create-knowledge-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📚knowledge` per Wave C.

use super::mutation::CreateKnowledgeRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Reads the live `knowledge` rows off the working-scene cache; Fatal `mutation.duplicate-id`
/// if the id already exists (empty diff); else appends the payload row and re-mints a fresh
/// content-addressed `table` child handle — composed-child equivalent of the former
/// `added = [payload row]` sparse delta (`📓️migration-recipe.md` §3/§4).
pub fn diff(payload: &CreateKnowledgeRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let mut records = crate::artifacts::program::program_knowledge(base);
    if records.iter().any(|row| row.header.id == payload.knowledge_record.header.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A knowledge record already exists with this id.", [payload.knowledge_record.header.id.0.clone()]);
    }
    records.push(payload.knowledge_record.clone());
    protocol::MutationOutcome::new(ProgramDiff { knowledge: Some(crate::artifacts::program::knowledge_child_from_records(&records)), ..Default::default() })
}
