//! ↩️ Inverse (undo) construction for the `create-register-row` leaf — computed from captured pre-state (`base`), never by structurally inverting the diff.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added from its register.
pub fn inverse(payload: &super::CreateRegisterRow, _base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok(vec![ProgramMutation::DeleteRegisterRow(super::super::delete_register_row::DeleteRegisterRow { register: payload.row.key(), id: payload.row.header().id.clone() })])
}
