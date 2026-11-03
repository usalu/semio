//! ↩️ `change-joint-base-normal-force` inverse: restores the previous `base_normal_force`.

use crate::change_joint_base_normal_force::ChangeJointBaseNormalForce;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointBaseNormalForce, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_base_normal_force::ChangeJointBaseNormalForce> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointBaseNormalForce { id: payload.id.clone(), new_base_normal_force: row.base_normal_force })],
        None => Vec::new(),
    })
}
