//! ↩️ `change-joint-bolt-rows` inverse: restores the previous `bolt_rows`.

use crate::change_joint_bolt_rows::ChangeJointBoltRows;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointBoltRows, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_bolt_rows::ChangeJointBoltRows> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointBoltRows { id: payload.id.clone(), new_bolt_rows: row.bolt_rows })],
        None => Vec::new(),
    })
}
