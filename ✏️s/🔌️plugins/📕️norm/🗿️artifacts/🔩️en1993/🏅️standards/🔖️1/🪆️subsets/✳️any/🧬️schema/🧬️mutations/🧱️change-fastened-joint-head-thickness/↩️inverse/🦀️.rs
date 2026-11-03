//! ↩️ `change-fastened-joint-head-thickness` inverse: restores the previous `head_thickness`.

use crate::change_fastened_joint_head_thickness::ChangeFastenedJointHeadThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFastenedJointHeadThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastened_joint_head_thickness::ChangeFastenedJointHeadThickness> {
    Ok(match base.fastened_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFastenedJointHeadThickness { id: payload.id.clone(), new_head_thickness: row.head_thickness })],
        None => Vec::new(),
    })
}
