//! ↩️ Inverse (undo) construction for the `replace-environmental-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🌿environmental` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::ReplaceEnvironmentalRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.environmental.iter().find(|row| row.header.id == payload.environmental_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceEnvironmentalRequirement(super::mutation::ReplaceEnvironmentalRequirement { environmental_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
