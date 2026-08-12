//! ↩️ Inverse (undo) construction for the `replace-human-factor-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧠human-factors` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::ReplaceHumanFactorRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.human_factors.iter().find(|row| row.header.id == payload.human_factor_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceHumanFactorRequirement(super::mutation::ReplaceHumanFactorRequirement { human_factor_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
