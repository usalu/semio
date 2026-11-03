//! ↩️ `change-joint-gauge` inverse: restores the previous `gauge`.

use crate::change_joint_gauge::ChangeJointGauge;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointGauge, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_gauge::ChangeJointGauge> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointGauge { id: payload.id.clone(), new_gauge: row.gauge })],
        None => Vec::new(),
    })
}
