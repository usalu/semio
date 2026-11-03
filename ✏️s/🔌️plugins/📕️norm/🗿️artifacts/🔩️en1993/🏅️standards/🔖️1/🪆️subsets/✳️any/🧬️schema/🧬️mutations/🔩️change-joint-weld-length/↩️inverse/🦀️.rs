//! ↩️ `change-joint-weld-length` inverse: restores the previous `weld_length`.

use crate::change_joint_weld_length::ChangeJointWeldLength;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointWeldLength, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_weld_length::ChangeJointWeldLength> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointWeldLength { id: payload.id.clone(), new_weld_length: row.weld_length })],
        None => Vec::new(),
    })
}
