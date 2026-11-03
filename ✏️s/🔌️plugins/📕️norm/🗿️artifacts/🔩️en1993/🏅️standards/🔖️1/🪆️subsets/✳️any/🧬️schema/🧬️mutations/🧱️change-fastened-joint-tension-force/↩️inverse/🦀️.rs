//! ↩️ `change-fastened-joint-tension-force` inverse: restores the previous `tension_force`.

use crate::change_fastened_joint_tension_force::ChangeFastenedJointTensionForce;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFastenedJointTensionForce, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastened_joint_tension_force::ChangeFastenedJointTensionForce> {
    Ok(match base.fastened_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFastenedJointTensionForce { id: payload.id.clone(), new_tension_force: row.tension_force })],
        None => Vec::new(),
    })
}
