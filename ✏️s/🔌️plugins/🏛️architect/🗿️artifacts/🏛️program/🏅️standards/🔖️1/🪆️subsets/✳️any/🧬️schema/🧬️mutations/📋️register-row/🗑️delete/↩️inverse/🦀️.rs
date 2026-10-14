//! ↩️ Inverse (undo) construction for the `delete-register-row` leaf — computed from captured pre-state (`base`), never by structurally inverting the diff.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row at its position. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteRegisterRow, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok(match payload.register.position(base, &payload.id).and_then(|position| payload.register.row_at(base, position).map(|row| (position, row))) {
        Some((position, row)) => vec![ProgramMutation::CreateRegisterRow(super::super::create_register_row::CreateRegisterRow { row, index: Some(position) })],
        None => Vec::new(),
    })
}
