//! ↩️ `change-joint-weld-force-longitudinal` inverse: restores the previous `weld_force_longitudinal`.

use crate::change_joint_weld_force_longitudinal::ChangeJointWeldForceLongitudinal;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointWeldForceLongitudinal, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_weld_force_longitudinal::ChangeJointWeldForceLongitudinal> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointWeldForceLongitudinal { id: payload.id.clone(), new_weld_force_longitudinal: row.weld_force_longitudinal })],
        None => Vec::new(),
    })
}
