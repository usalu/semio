//! ↩️ `change-fastened-joint-washer-diameter` inverse: restores the previous `washer_diameter`.

use crate::change_fastened_joint_washer_diameter::ChangeFastenedJointWasherDiameter;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFastenedJointWasherDiameter, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastened_joint_washer_diameter::ChangeFastenedJointWasherDiameter> {
    Ok(match base.fastened_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFastenedJointWasherDiameter { id: payload.id.clone(), new_washer_diameter: row.washer_diameter })],
        None => Vec::new(),
    })
}
