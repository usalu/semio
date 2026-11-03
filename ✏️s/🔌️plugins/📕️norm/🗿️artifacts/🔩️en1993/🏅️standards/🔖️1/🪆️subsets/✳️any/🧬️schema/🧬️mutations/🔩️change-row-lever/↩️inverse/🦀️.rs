//! ↩️ `change-row-lever` inverse: restores the previous `lever`.

use crate::change_row_lever::ChangeRowLever;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRowLever, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_row_lever::ChangeRowLever> {
    Ok(match base.joint_rows.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRowLever { id: payload.id.clone(), new_lever: row.lever })],
        None => Vec::new(),
    })
}
