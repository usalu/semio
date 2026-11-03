//! ↩️ `change-joint-bolt-diameter` inverse: restores the previous `bolt_diameter`.

use crate::change_joint_bolt_diameter::ChangeJointBoltDiameter;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointBoltDiameter, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_bolt_diameter::ChangeJointBoltDiameter> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointBoltDiameter { id: payload.id.clone(), new_bolt_diameter: row.bolt_diameter })],
        None => Vec::new(),
    })
}
