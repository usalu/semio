//! ↩️ `change-joint-anchor-area` inverse: restores the previous `anchor_area`.

use crate::change_joint_anchor_area::ChangeJointAnchorArea;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointAnchorArea, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_anchor_area::ChangeJointAnchorArea> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointAnchorArea { id: payload.id.clone(), new_anchor_area: row.anchor_area })],
        None => Vec::new(),
    })
}
