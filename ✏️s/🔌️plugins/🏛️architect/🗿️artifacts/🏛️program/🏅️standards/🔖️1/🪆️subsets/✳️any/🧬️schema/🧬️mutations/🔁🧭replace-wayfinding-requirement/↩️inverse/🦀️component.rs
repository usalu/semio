//! ↩️ Inverse (undo) construction for the `replace-wayfinding-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧭wayfinding` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::ReplaceWayfindingRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.wayfinding.iter().find(|row| row.header.id == payload.wayfinding_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceWayfindingRequirement(super::mutation::ReplaceWayfindingRequirement { wayfinding_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
