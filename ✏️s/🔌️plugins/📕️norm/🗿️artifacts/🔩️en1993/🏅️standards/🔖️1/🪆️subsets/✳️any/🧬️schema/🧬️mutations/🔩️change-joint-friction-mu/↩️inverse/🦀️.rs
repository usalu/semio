//! ↩️ `change-joint-friction-mu` inverse: restores the previous `friction_mu`.

use crate::change_joint_friction_mu::ChangeJointFrictionMu;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointFrictionMu, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_friction_mu::ChangeJointFrictionMu> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointFrictionMu { id: payload.id.clone(), new_friction_mu: row.friction_mu })],
        None => Vec::new(),
    })
}
