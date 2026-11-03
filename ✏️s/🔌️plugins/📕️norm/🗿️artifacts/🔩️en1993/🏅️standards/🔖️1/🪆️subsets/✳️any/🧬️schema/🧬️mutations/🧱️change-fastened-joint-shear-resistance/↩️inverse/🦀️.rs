//! ↩️ `change-fastened-joint-shear-resistance` inverse: restores the previous `shear_resistance`.

use crate::change_fastened_joint_shear_resistance::ChangeFastenedJointShearResistance;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFastenedJointShearResistance, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastened_joint_shear_resistance::ChangeFastenedJointShearResistance> {
    Ok(match base.fastened_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFastenedJointShearResistance { id: payload.id.clone(), new_shear_resistance: row.shear_resistance })],
        None => Vec::new(),
    })
}
