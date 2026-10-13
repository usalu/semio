//! ↩️ Inverse (undo) construction for the `create-validation-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `✔️validations` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub fn inverse(payload: &super::CreateValidationRecord, _base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ProgramMutation::DeleteValidationRecord(super::super::delete_validation_record::DeleteValidationRecord { id: payload.validation_record.header.id.clone() })]

    })())
}
