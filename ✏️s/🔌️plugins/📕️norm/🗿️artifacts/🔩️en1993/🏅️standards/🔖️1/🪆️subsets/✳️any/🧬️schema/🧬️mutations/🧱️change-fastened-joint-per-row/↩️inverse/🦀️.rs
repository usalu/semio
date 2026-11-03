//! ↩️ `change-fastened-joint-per-row` inverse: restores the previous `per_row`.

use crate::change_fastened_joint_per_row::ChangeFastenedJointPerRow;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFastenedJointPerRow, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastened_joint_per_row::ChangeFastenedJointPerRow> {
    Ok(match base.fastened_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFastenedJointPerRow { id: payload.id.clone(), new_per_row: row.per_row })],
        None => Vec::new(),
    })
}
