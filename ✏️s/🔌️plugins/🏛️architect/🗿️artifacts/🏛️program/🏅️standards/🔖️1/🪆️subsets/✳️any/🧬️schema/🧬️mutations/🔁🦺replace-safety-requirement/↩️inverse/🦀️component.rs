//! ↩️ Inverse (undo) construction for the `replace-safety-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🦺safety` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::ReplaceSafetyRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.safety.iter().find(|row| row.header.id == payload.safety_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceSafetyRequirement(super::mutation::ReplaceSafetyRequirement { safety_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
