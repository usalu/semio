//! ↩️ Inverse (undo) construction for the `replace-validation-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `✔️validations` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::ReplaceValidationRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.validations.iter().find(|row| row.header.id == payload.validation_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceValidationRecord(super::mutation::ReplaceValidationRecord { validation_record: existing.clone() })],
        None => Vec::new(),
    }
}
