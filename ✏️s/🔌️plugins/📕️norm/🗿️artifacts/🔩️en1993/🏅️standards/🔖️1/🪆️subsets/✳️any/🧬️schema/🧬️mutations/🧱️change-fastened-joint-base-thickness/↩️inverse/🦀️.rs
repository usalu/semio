//! ↩️ `change-fastened-joint-base-thickness` inverse: restores the previous `base_thickness`.

use crate::change_fastened_joint_base_thickness::ChangeFastenedJointBaseThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFastenedJointBaseThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastened_joint_base_thickness::ChangeFastenedJointBaseThickness> {
    Ok(match base.fastened_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFastenedJointBaseThickness { id: payload.id.clone(), new_base_thickness: row.base_thickness })],
        None => Vec::new(),
    })
}
