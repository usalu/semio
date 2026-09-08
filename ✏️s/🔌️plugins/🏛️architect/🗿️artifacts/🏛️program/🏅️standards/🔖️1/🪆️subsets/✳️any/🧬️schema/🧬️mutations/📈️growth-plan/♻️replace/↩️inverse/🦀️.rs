//! ↩️ Inverse (undo) construction for the `replace-growth-plan` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📈growth` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::ReplaceGrowthPlan, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.growth.iter().find(|row| row.header.id == payload.growth_plan.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceGrowthPlan(super::ReplaceGrowthPlan { growth_plan: existing.clone() })],
        None => Vec::new(),
    }
}
