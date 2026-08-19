//! ↩️ Inverse (undo) construction for the `create-validation-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `✔️validations` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::mutation::CreateValidationRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteValidationRecord(super::super::delete_validation_record::mutation::DeleteValidationRecord { id: payload.validation_record.header.id.clone() })]
}
