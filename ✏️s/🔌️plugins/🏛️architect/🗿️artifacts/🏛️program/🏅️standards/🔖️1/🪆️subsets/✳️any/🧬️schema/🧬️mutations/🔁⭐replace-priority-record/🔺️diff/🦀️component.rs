//! 🔺️ Sparse diff construction for the `replace-priority-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `⭐priorities` per Wave C.

use super::mutation::ReplacePriorityRecord;
use crate::artifacts::program::diff::{ProgramPrioritiesDelta, ProgramPrioritiesPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplacePriorityRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.priorities.iter().find(|row| row.header.id == payload.priority_record.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No priority record exists with this id.", [payload.priority_record.header.id.0.clone()]);
    };
    if existing == &payload.priority_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This priority record already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.priority_record).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { priorities: Some(ProgramPrioritiesDelta { patched: vec![ProgramPrioritiesPatchEntry { id: payload.priority_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
