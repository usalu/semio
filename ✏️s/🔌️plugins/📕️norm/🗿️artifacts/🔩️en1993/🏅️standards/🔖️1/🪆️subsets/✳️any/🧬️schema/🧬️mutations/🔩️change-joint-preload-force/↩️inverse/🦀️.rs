//! ↩️ `change-joint-preload-force` inverse: restores the previous `preload_force`.

use crate::change_joint_preload_force::ChangeJointPreloadForce;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointPreloadForce, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_preload_force::ChangeJointPreloadForce> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointPreloadForce { id: payload.id.clone(), new_preload_force: row.preload_force })],
        None => Vec::new(),
    })
}
