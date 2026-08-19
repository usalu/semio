//! 🔺️ Sparse diff construction for the `create-priority-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⭐priorities` per Wave C.

use super::mutation::CreatePriorityRecord;
use crate::artifacts::program::diff::ProgramPrioritiesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub async fn diff(payload: &CreatePriorityRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.priority_record.header.id.clone();
    if base.priorities.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A priority record already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { priorities: Some(ProgramPrioritiesDelta { added: vec![payload.priority_record.clone()], ..Default::default() }), ..Default::default() })
}
