//! ↩️ `change-joint-bolt-stress-area` inverse: restores the previous `bolt_stress_area`.

use crate::change_joint_bolt_stress_area::ChangeJointBoltStressArea;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointBoltStressArea, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_bolt_stress_area::ChangeJointBoltStressArea> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointBoltStressArea { id: payload.id.clone(), new_bolt_stress_area: row.bolt_stress_area })],
        None => Vec::new(),
    })
}
