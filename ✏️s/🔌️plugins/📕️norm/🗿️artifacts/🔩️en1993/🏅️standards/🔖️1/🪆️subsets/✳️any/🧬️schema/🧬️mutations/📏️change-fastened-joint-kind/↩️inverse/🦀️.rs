//! ↩️ `change-fastened-joint-fastened joint` inverse: restores the previous `fastened joint`.

use crate::change_fastened_joint_kind::ChangeFastenedJointKind;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFastenedJointKind, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastened_joint_kind::ChangeFastenedJointKind> {
    Ok(match base.fastened_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFastenedJointKind { id: payload.id.clone(), new_kind: row.kind })],
        None => Vec::new(),
    })
}
