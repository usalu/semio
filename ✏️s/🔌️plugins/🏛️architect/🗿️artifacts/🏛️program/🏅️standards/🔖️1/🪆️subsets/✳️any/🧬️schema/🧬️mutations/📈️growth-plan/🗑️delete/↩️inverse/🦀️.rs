//! ↩️ Inverse (undo) construction for the `delete-growth-plan` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📈growth` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteGrowthPlan, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.growth.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateGrowthPlan(super::super::create_growth_plan::CreateGrowthPlan { growth_plan: base.growth[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
