//! ↩️ `change-row-grip-length` inverse: restores the previous `grip_length`.

use crate::change_row_grip_length::ChangeRowGripLength;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRowGripLength, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_row_grip_length::ChangeRowGripLength> {
    Ok(match base.joint_rows.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRowGripLength { id: payload.id.clone(), new_grip_length: row.grip_length })],
        None => Vec::new(),
    })
}
