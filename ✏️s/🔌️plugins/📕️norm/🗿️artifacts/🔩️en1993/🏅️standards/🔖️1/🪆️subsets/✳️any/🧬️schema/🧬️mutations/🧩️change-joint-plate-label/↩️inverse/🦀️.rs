//! ↩️ `change-joint-plate-label` inverse: restores the previous `label`.

use crate::change_joint_plate_label::ChangeJointPlateLabel;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointPlateLabel, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_plate_label::ChangeJointPlateLabel> {
    Ok(match base.joint_plates.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointPlateLabel { id: payload.id.clone(), new_label: row.label.clone() })],
        None => Vec::new(),
    })
}
