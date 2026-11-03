//! ↩️ `change-fastened-joint-head-strength` inverse: restores the previous `head_strength`.

use crate::change_fastened_joint_head_strength::ChangeFastenedJointHeadStrength;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFastenedJointHeadStrength, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastened_joint_head_strength::ChangeFastenedJointHeadStrength> {
    Ok(match base.fastened_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFastenedJointHeadStrength { id: payload.id.clone(), new_head_strength: row.head_strength })],
        None => Vec::new(),
    })
}
