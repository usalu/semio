//! ↩️ Inverse (undo) construction for the `rename-validation-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `✔️validations` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a rename by restoring the pre-state name. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::mutation::RenameValidationRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.validations.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::RenameValidationRecord(super::mutation::RenameValidationRecord { id: payload.id.clone(), new_name: existing.header.name.clone() })],
        None => Vec::new(),
    }
}
