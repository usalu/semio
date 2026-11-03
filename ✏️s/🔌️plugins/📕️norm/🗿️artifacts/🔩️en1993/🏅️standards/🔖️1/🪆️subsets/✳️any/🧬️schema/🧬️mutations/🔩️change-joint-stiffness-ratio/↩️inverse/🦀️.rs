//! ↩️ `change-joint-stiffness-ratio` inverse: restores the previous `stiffness_ratio`.

use crate::change_joint_stiffness_ratio::ChangeJointStiffnessRatio;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointStiffnessRatio, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_stiffness_ratio::ChangeJointStiffnessRatio> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointStiffnessRatio { id: payload.id.clone(), new_stiffness_ratio: row.stiffness_ratio })],
        None => Vec::new(),
    })
}
