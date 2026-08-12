//! ↩️ Inverse (undo) construction for the `replace-sustainability-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `♻️sustainability` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::ReplaceSustainabilityRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.sustainability.iter().find(|row| row.header.id == payload.sustainability_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceSustainabilityRequirement(super::mutation::ReplaceSustainabilityRequirement { sustainability_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
