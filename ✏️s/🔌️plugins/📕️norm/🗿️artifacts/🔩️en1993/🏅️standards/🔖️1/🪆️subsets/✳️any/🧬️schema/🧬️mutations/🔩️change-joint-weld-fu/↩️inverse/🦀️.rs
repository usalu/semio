//! ↩️ `change-joint-weld-fu` inverse: restores the previous `weld_fu`.

use crate::change_joint_weld_fu::ChangeJointWeldFu;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointWeldFu, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_weld_fu::ChangeJointWeldFu> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointWeldFu { id: payload.id.clone(), new_weld_fu: row.weld_fu })],
        None => Vec::new(),
    })
}
