//! ↩️ `change-joint-braced-frame` inverse: restores the previous `braced_frame`.

use crate::change_joint_braced_frame::ChangeJointBracedFrame;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointBracedFrame, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_braced_frame::ChangeJointBracedFrame> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointBracedFrame { id: payload.id.clone(), new_braced_frame: row.braced_frame })],
        None => Vec::new(),
    })
}
