//! ↩️ `change-row-row-type` inverse: restores the previous `row_type`.

use crate::change_row_row_type::ChangeRowRowType;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRowRowType, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_row_row_type::ChangeRowRowType> {
    Ok(match base.joint_rows.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRowRowType { id: payload.id.clone(), new_row_type: row.row_type })],
        None => Vec::new(),
    })
}
