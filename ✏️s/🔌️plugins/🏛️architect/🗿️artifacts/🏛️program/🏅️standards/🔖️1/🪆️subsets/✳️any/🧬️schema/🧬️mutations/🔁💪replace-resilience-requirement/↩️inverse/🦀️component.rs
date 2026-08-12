//! ↩️ Inverse (undo) construction for the `replace-resilience-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `💪resilience` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::ReplaceResilienceRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.resilience.iter().find(|row| row.header.id == payload.resilience_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceResilienceRequirement(super::mutation::ReplaceResilienceRequirement { resilience_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
