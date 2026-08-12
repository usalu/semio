//! ↩️ Inverse (undo) construction for the `replace-cost-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `💰costs` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::ReplaceCostRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.costs.iter().find(|row| row.header.id == payload.cost_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceCostRequirement(super::mutation::ReplaceCostRequirement { cost_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
