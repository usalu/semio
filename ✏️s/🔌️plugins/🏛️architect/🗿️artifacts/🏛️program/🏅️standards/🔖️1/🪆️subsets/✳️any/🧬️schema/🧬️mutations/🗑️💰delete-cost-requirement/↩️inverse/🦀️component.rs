//! ↩️ Inverse (undo) construction for the `delete-cost-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `💰costs` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::DeleteCostRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.costs.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateCostRequirement(super::super::create_cost_requirement::mutation::CreateCostRequirement { cost_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
