//! ↩️ `change-fastened-joint-sheet-width` inverse: restores the previous `sheet_width`.

use crate::change_fastened_joint_sheet_width::ChangeFastenedJointSheetWidth;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFastenedJointSheetWidth, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastened_joint_sheet_width::ChangeFastenedJointSheetWidth> {
    Ok(match base.fastened_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFastenedJointSheetWidth { id: payload.id.clone(), new_sheet_width: row.sheet_width })],
        None => Vec::new(),
    })
}
