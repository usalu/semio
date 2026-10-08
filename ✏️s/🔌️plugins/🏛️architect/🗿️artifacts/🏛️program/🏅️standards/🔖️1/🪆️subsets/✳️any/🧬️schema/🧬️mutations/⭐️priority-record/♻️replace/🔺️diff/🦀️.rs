//! 🔺️ Sparse diff construction for the `replace-priority-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⭐priorities` per Wave C.

use super::ReplacePriorityRecord;
use crate::diff::ProgramPrioritiesDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns:
/// `removed = [id]`, `added = [payload row]`, and `reordered` (the base order) unless the row was last, so the new row keeps its position.
pub fn diff(payload: &ReplacePriorityRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.priority_record.header.id;
    let Some(position) = base.priorities.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No priority record exists with this id.", [id.0.clone()]);
    };
    if base.priorities[position] == payload.priority_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This priority record already matches the requested value.").at([id.0.clone()])]);
    }
    let reordered = (position + 1 != base.priorities.len()).then(|| base.priorities.iter().map(|row| row.header.id.0.clone()).collect());
    protocol::MutationOutcome::new(ProgramDiff { priorities: Some(ProgramPrioritiesDelta { removed: vec![id.0.clone()], added: vec![payload.priority_record.clone()], reordered, ..Default::default() }), ..Default::default() })
}
