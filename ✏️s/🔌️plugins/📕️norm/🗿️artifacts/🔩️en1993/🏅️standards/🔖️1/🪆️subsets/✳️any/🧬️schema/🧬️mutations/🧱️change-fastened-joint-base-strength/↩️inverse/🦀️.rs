//! ↩️ `change-fastened-joint-base-strength` inverse: restores the previous `base_strength`.

use crate::change_fastened_joint_base_strength::ChangeFastenedJointBaseStrength;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFastenedJointBaseStrength, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastened_joint_base_strength::ChangeFastenedJointBaseStrength> {
    Ok(match base.fastened_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFastenedJointBaseStrength { id: payload.id.clone(), new_base_strength: row.base_strength })],
        None => Vec::new(),
    })
}
