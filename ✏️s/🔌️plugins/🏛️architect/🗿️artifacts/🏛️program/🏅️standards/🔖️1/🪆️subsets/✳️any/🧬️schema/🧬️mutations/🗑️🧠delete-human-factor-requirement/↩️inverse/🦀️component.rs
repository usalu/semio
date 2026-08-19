//! ↩️ Inverse (undo) construction for the `delete-human-factor-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧠human-factors` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::DeleteHumanFactorRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.human_factors.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateHumanFactorRequirement(super::super::create_human_factor_requirement::mutation::CreateHumanFactorRequirement { human_factor_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
