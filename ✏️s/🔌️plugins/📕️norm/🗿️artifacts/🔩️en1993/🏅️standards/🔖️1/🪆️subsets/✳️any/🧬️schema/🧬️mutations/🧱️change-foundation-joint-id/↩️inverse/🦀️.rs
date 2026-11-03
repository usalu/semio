//! ↩️ `change-foundation-joint-id` inverse: restores the previous `joint_id`.

use crate::change_foundation_joint_id::ChangeFoundationJointId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationJointId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_joint_id::ChangeFoundationJointId> {
    Ok(match base.foundation_blocks.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFoundationJointId { id: payload.id.clone(), new_joint_id: row.joint_id.clone() })],
        None => Vec::new(),
    })
}
