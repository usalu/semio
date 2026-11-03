//! ↩️ `change-joint-kind` inverse: restores the previous `kind`.

use crate::change_joint_kind::ChangeJointKind;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointKind, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_kind::ChangeJointKind> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointKind { id: payload.id.clone(), new_kind: row.kind })],
        None => Vec::new(),
    })
}
