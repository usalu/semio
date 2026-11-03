//! ↩️ `change-built-up-diagonal-length` inverse: restores the previous `diagonal_length`.

use crate::change_built_up_diagonal_length::ChangeBuiltUpDiagonalLength;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBuiltUpDiagonalLength, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_built_up_diagonal_length::ChangeBuiltUpDiagonalLength> {
    Ok(match base.built_up_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBuiltUpDiagonalLength { id: payload.id.clone(), new_diagonal_length: row.diagonal_length })],
        None => Vec::new(),
    })
}
