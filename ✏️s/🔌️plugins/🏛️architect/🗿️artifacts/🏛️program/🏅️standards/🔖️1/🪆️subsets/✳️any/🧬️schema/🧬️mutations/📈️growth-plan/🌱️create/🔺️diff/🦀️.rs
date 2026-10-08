//! 🔺️ Sparse diff construction for the `create-growth-plan` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📈growth` per Wave C.

use super::CreateGrowthPlan;
use crate::diff::ProgramGrowthDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateGrowthPlan, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.growth_plan.header.id;
    if base.growth.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A growth plan already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.growth.len());
    if at > base.growth.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the growth plan list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { growth: Some(ProgramGrowthDelta::insertion(at, payload.growth_plan.clone())), ..Default::default() })
}
