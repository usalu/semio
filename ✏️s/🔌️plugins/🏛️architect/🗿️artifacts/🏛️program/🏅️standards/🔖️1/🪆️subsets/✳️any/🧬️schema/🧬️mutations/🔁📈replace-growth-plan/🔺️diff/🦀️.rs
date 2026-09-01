//! 🔺️ Sparse diff construction for the `replace-growth-plan` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📈growth` per Wave C.

use super::ReplaceGrowthPlan;
use crate::artifacts::program::diff::{ProgramGrowthDelta, ProgramGrowthPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceGrowthPlan, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.growth.iter().find(|row| row.header.id == payload.growth_plan.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No growth plan exists with this id.", [payload.growth_plan.header.id.0.clone()]);
    };
    if existing == &payload.growth_plan {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This growth plan already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.growth_plan).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { growth: Some(ProgramGrowthDelta { patched: vec![ProgramGrowthPatchEntry { id: payload.growth_plan.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
