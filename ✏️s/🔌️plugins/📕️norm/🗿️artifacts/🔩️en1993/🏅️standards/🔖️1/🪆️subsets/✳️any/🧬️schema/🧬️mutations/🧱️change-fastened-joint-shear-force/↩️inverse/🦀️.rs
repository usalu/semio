//! ↩️ `change-fastened-joint-shear-force` inverse: restores the previous `shear_force`.

use crate::change_fastened_joint_shear_force::ChangeFastenedJointShearForce;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFastenedJointShearForce, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastened_joint_shear_force::ChangeFastenedJointShearForce> {
    Ok(match base.fastened_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFastenedJointShearForce { id: payload.id.clone(), new_shear_force: row.shear_force })],
        None => Vec::new(),
    })
}
