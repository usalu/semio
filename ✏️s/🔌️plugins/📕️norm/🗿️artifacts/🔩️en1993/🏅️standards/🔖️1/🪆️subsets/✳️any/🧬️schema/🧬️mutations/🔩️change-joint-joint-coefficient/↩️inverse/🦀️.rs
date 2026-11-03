//! ↩️ `change-joint-joint-coefficient` inverse: restores the previous `joint_coefficient`.

use crate::change_joint_joint_coefficient::ChangeJointJointCoefficient;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointJointCoefficient, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_joint_coefficient::ChangeJointJointCoefficient> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointJointCoefficient { id: payload.id.clone(), new_joint_coefficient: row.joint_coefficient })],
        None => Vec::new(),
    })
}
