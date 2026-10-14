//! ↩️ Inverse (undo) construction for the `replace-register-row` leaf — computed from captured pre-state (`base`), never by structurally inverting the diff.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::ReplaceRegisterRow, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    let key = payload.row.key();
    Ok(match key.position(base, &payload.row.header().id).and_then(|position| key.row_at(base, position)) {
        Some(row) => vec![ProgramMutation::ReplaceRegisterRow(super::ReplaceRegisterRow { row })],
        None => Vec::new(),
    })
}
