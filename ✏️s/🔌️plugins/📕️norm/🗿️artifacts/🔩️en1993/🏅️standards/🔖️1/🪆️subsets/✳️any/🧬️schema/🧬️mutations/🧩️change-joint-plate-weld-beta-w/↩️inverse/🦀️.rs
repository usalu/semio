//! ↩️ `change-joint-plate-weld-beta-w` inverse: restores the previous `weld_beta_w`.

use crate::change_joint_plate_weld_beta_w::ChangeJointPlateWeldBetaW;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointPlateWeldBetaW, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_plate_weld_beta_w::ChangeJointPlateWeldBetaW> {
    Ok(match base.joint_plates.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointPlateWeldBetaW { id: payload.id.clone(), new_weld_beta_w: row.weld_beta_w })],
        None => Vec::new(),
    })
}
