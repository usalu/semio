//! ↩️ Inverse (undo) construction for the `rename-information-requirement` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `ℹ️information` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::RenameInformationRequirement, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.information.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameInformationRequirement(super::RenameInformationRequirement { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}
