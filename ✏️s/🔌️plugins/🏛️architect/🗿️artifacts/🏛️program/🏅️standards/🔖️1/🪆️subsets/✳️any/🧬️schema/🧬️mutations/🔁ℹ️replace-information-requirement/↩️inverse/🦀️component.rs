//! ↩️ Inverse (undo) construction for the `replace-information-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `ℹ️information` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::ReplaceInformationRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.information.iter().find(|row| row.header.id == payload.information_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceInformationRequirement(super::mutation::ReplaceInformationRequirement { information_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
