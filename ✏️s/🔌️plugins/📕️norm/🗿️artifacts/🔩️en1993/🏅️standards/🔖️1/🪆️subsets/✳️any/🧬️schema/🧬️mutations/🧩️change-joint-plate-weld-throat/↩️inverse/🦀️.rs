//! ↩️ `change-joint-plate-weld-throat` inverse: restores the previous `weld_throat`.

use crate::change_joint_plate_weld_throat::ChangeJointPlateWeldThroat;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointPlateWeldThroat, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_plate_weld_throat::ChangeJointPlateWeldThroat> {
    Ok(match base.joint_plates.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointPlateWeldThroat { id: payload.id.clone(), new_weld_throat: row.weld_throat })],
        None => Vec::new(),
    })
}
