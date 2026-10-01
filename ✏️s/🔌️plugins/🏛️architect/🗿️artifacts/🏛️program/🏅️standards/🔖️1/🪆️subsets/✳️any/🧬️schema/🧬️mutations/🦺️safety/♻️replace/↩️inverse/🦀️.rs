//! ↩️ Inverse (undo) construction for the `replace-safety-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🦺safety` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::ReplaceSafetyRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.safety.iter().find(|row| row.header.id == payload.safety_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceSafetyRequirement(super::ReplaceSafetyRequirement { safety_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
