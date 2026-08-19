//! 🔺️ Sparse diff construction for the `rename-growth-plan` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📈growth` per Wave C.

use super::mutation::RenameGrowthPlan;
use crate::artifacts::program::diff::{ProgramGrowthDelta, ProgramGrowthPatchEntry};
use crate::artifacts::program::registers::GrowthPlanPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// ✏️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the name is unchanged (both empty diff), else `patched = [{id, name: Some(new_name)}]`.
pub async fn diff(payload: &RenameGrowthPlan, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.growth.iter().find(|row| row.header.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No growth plan exists with this id.", [payload.id.0.clone()]);
    };
    if existing.header.name == payload.new_name {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This growth plan already has this name.").at([payload.id.0.clone()])]);
    }
    let patch = GrowthPlanPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    protocol::MutationOutcome::new(ProgramDiff { growth: Some(ProgramGrowthDelta { patched: vec![ProgramGrowthPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
