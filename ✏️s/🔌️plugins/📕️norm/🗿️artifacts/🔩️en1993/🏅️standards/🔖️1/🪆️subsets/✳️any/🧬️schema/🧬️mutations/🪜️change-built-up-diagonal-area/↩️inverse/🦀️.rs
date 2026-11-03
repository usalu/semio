//! ↩️ `change-built-up-diagonal-area` inverse: restores the previous `diagonal_area`.

use crate::change_built_up_diagonal_area::ChangeBuiltUpDiagonalArea;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBuiltUpDiagonalArea, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_built_up_diagonal_area::ChangeBuiltUpDiagonalArea> {
    Ok(match base.built_up_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBuiltUpDiagonalArea { id: payload.id.clone(), new_diagonal_area: row.diagonal_area })],
        None => Vec::new(),
    })
}
