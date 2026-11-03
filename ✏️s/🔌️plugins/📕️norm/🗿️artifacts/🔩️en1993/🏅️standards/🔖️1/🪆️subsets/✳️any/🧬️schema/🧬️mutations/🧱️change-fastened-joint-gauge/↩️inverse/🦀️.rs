//! ↩️ `change-fastened-joint-gauge` inverse: restores the previous `gauge`.

use crate::change_fastened_joint_gauge::ChangeFastenedJointGauge;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFastenedJointGauge, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastened_joint_gauge::ChangeFastenedJointGauge> {
    Ok(match base.fastened_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFastenedJointGauge { id: payload.id.clone(), new_gauge: row.gauge })],
        None => Vec::new(),
    })
}
