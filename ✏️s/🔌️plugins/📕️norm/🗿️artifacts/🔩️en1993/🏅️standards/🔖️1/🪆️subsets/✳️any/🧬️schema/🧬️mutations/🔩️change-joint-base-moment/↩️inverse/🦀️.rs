//! ↩️ `change-joint-base-moment` inverse: restores the previous `base_moment`.

use crate::change_joint_base_moment::ChangeJointBaseMoment;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointBaseMoment, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_base_moment::ChangeJointBaseMoment> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointBaseMoment { id: payload.id.clone(), new_base_moment: row.base_moment })],
        None => Vec::new(),
    })
}
