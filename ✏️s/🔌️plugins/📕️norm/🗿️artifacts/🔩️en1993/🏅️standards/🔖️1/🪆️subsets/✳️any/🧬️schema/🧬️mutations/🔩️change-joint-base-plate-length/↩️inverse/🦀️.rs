//! ↩️ `change-joint-base-plate-length` inverse: restores the previous `base_plate_length`.

use crate::change_joint_base_plate_length::ChangeJointBasePlateLength;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointBasePlateLength, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_base_plate_length::ChangeJointBasePlateLength> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointBasePlateLength { id: payload.id.clone(), new_base_plate_length: row.base_plate_length })],
        None => Vec::new(),
    })
}
