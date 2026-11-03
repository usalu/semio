//! ↩️ `change-joint-bolt-grip-length` inverse: restores the previous `bolt_grip_length`.

use crate::change_joint_bolt_grip_length::ChangeJointBoltGripLength;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointBoltGripLength, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_bolt_grip_length::ChangeJointBoltGripLength> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointBoltGripLength { id: payload.id.clone(), new_bolt_grip_length: row.bolt_grip_length })],
        None => Vec::new(),
    })
}
