//! ↩️ `change-fastened-joint-count` inverse: restores the previous `count`.

use crate::change_fastened_joint_count::ChangeFastenedJointCount;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFastenedJointCount, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastened_joint_count::ChangeFastenedJointCount> {
    Ok(match base.fastened_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFastenedJointCount { id: payload.id.clone(), new_count: row.count })],
        None => Vec::new(),
    })
}
