//! 🔺️ Sparse diff construction for the `create-growth-plan` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📈growth` per Wave C.

use super::mutation::CreateGrowthPlan;
use crate::artifacts::program::diff::ProgramGrowthDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub async fn diff(payload: &CreateGrowthPlan, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.growth_plan.header.id.clone();
    if base.growth.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A growth plan already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { growth: Some(ProgramGrowthDelta { added: vec![payload.growth_plan.clone()], ..Default::default() }), ..Default::default() })
}
