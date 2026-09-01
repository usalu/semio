//! ↩️ Inverse (undo) construction for the `delete-growth-plan` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📈growth` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::DeleteGrowthPlan, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.growth.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateGrowthPlan(super::super::create_growth_plan::CreateGrowthPlan { growth_plan: existing.clone() })],
        None => Vec::new(),
    }
}
