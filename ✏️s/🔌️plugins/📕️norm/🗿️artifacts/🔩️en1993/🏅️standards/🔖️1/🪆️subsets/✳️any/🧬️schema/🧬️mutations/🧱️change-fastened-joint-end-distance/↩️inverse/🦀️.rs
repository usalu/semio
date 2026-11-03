//! ↩️ `change-fastened-joint-end-distance` inverse: restores the previous `end_distance`.

use crate::change_fastened_joint_end_distance::ChangeFastenedJointEndDistance;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFastenedJointEndDistance, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastened_joint_end_distance::ChangeFastenedJointEndDistance> {
    Ok(match base.fastened_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFastenedJointEndDistance { id: payload.id.clone(), new_end_distance: row.end_distance })],
        None => Vec::new(),
    })
}
