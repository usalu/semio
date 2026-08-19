//! ↩️ Inverse (undo) construction for the `create-growth-plan` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📈growth` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::mutation::CreateGrowthPlan, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteGrowthPlan(super::super::delete_growth_plan::mutation::DeleteGrowthPlan { id: payload.growth_plan.header.id.clone() })]
}
