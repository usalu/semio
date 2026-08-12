//! ↩️ Inverse (undo) construction for the `delete-information-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `ℹ️information` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::DeleteInformationRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.information.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateInformationRequirement(super::super::create_information_requirement::mutation::CreateInformationRequirement { information_requirement: existing.clone() })],
        None => Vec::new(),
    }
}
