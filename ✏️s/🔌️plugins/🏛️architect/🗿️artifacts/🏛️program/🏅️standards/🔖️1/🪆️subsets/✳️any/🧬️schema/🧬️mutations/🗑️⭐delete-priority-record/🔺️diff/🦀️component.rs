//! 🔺️ Sparse diff construction for the `delete-priority-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⭐priorities` per Wave C.

use super::mutation::DeletePriorityRecord;
use crate::artifacts::program::diff::ProgramPrioritiesDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
pub fn diff(payload: &DeletePriorityRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    if !base.priorities.iter().any(|row| row.header.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", "No priority record exists with this id.", [payload.id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { priorities: Some(ProgramPrioritiesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() })
}
