//! ↩️ `change-row-group-with-previous` inverse: restores the previous `group_with_previous`.

use crate::change_row_group_with_previous::ChangeRowGroupWithPrevious;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRowGroupWithPrevious, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_row_group_with_previous::ChangeRowGroupWithPrevious> {
    Ok(match base.joint_rows.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRowGroupWithPrevious { id: payload.id.clone(), new_group_with_previous: row.group_with_previous })],
        None => Vec::new(),
    })
}
