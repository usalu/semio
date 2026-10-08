//! 🔺️ Sparse diff construction for the `create-priority-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⭐priorities` per Wave C.

use super::CreatePriorityRecord;
use crate::diff::ProgramPrioritiesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreatePriorityRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.priority_record.header.id;
    if base.priorities.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A priority record already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.priorities.len());
    if at > base.priorities.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the priority record list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { priorities: Some(ProgramPrioritiesDelta::insertion(at, payload.priority_record.clone())), ..Default::default() })
}
