//! ↩️ Inverse (undo) construction for the `replace-regulatory-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `📜regulatory` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::ReplaceRegulatoryRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.regulatory.iter().find(|row| row.header.id == payload.regulatory_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceRegulatoryRequirement(super::mutation::ReplaceRegulatoryRequirement { regulatory_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
