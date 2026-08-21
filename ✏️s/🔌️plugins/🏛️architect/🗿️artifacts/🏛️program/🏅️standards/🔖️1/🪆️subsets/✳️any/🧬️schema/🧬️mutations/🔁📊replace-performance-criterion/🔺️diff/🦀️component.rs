//! 🔺️ Sparse diff construction for the `replace-performance-criterion` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📊performance` per Wave C.

use super::mutation::ReplacePerformanceCriterion;
use crate::artifacts::program::diff::{ProgramPerformanceDelta, ProgramPerformancePatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplacePerformanceCriterion, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.performance.iter().find(|row| row.header.id == payload.performance_criterion.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No performance criterion exists with this id.", [payload.performance_criterion.header.id.0.clone()]);
    };
    if existing == &payload.performance_criterion {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This performance criterion already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.performance_criterion).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff {
        performance: Some(ProgramPerformanceDelta { patched: vec![ProgramPerformancePatchEntry { id: payload.performance_criterion.header.id.0.clone(), patch }], ..Default::default() }),
        ..Default::default()
    })
}
