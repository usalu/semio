//! ↩️ Inverse (undo) construction for the `create-cost-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `💰costs` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::mutation::CreateCostRequirement, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteCostRequirement(super::super::delete_cost_requirement::mutation::DeleteCostRequirement { id: payload.cost_requirement.header.id.clone() })]
}
