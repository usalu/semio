//! ↩️ Inverse (undo) construction for the `delete-cost-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `💰costs` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteCostRequirement, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.costs.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateCostRequirement(super::super::create_cost_requirement::CreateCostRequirement { cost_requirement: base.costs[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
