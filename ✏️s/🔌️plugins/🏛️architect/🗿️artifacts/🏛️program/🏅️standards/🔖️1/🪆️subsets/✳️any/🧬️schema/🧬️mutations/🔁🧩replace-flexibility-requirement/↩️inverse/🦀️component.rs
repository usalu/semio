//! ↩️ Inverse (undo) construction for the `replace-flexibility-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧩flexibility` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::ReplaceFlexibilityRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.flexibility.iter().find(|row| row.header.id == payload.flexibility_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceFlexibilityRequirement(super::mutation::ReplaceFlexibilityRequirement { flexibility_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
