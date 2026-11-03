//! ↩️ `change-joint-anchor-length` inverse: restores the previous `anchor_length`.

use crate::change_joint_anchor_length::ChangeJointAnchorLength;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointAnchorLength, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_anchor_length::ChangeJointAnchorLength> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointAnchorLength { id: payload.id.clone(), new_anchor_length: row.anchor_length })],
        None => Vec::new(),
    })
}
