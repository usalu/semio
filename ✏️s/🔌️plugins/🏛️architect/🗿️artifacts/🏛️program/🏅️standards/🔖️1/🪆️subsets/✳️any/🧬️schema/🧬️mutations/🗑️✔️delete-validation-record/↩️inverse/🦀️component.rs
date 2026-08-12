//! ↩️ Inverse (undo) construction for the `delete-validation-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `✔️validations` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::DeleteValidationRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.validations.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateValidationRecord(super::super::create_validation_record::mutation::CreateValidationRecord { validation_record: existing.clone() })],
        None => Vec::new(),
    }
}
