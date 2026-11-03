//! ↩️ `change-fastened-joint-diameter` inverse: restores the previous `diameter`.

use crate::change_fastened_joint_diameter::ChangeFastenedJointDiameter;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFastenedJointDiameter, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastened_joint_diameter::ChangeFastenedJointDiameter> {
    Ok(match base.fastened_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFastenedJointDiameter { id: payload.id.clone(), new_diameter: row.diameter })],
        None => Vec::new(),
    })
}
