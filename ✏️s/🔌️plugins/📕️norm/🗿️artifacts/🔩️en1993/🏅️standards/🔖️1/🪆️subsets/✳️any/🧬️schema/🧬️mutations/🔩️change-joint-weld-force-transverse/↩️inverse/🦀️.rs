//! ↩️ `change-joint-weld-force-transverse` inverse: restores the previous `weld_force_transverse`.

use crate::change_joint_weld_force_transverse::ChangeJointWeldForceTransverse;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointWeldForceTransverse, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_weld_force_transverse::ChangeJointWeldForceTransverse> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointWeldForceTransverse { id: payload.id.clone(), new_weld_force_transverse: row.weld_force_transverse })],
        None => Vec::new(),
    })
}
