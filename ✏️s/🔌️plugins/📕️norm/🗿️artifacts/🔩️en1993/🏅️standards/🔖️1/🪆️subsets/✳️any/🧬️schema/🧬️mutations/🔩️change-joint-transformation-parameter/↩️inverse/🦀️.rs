//! ↩️ `change-joint-transformation-parameter` inverse: restores the previous `transformation_parameter`.

use crate::change_joint_transformation_parameter::ChangeJointTransformationParameter;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointTransformationParameter, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_transformation_parameter::ChangeJointTransformationParameter> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointTransformationParameter { id: payload.id.clone(), new_transformation_parameter: row.transformation_parameter })],
        None => Vec::new(),
    })
}
