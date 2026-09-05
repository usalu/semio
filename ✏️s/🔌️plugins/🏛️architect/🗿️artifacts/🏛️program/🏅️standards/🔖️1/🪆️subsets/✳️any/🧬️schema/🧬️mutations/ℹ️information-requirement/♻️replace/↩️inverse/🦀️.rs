//! ↩️ Inverse (undo) construction for the `replace-information-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `ℹ️information` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::ReplaceInformationRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.information.iter().find(|row| row.header.id == payload.information_requirement.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceInformationRequirement(super::ReplaceInformationRequirement { information_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
