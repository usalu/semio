//! ↩️ `change-joint-column-web-depth` inverse: restores the previous `column_web_depth`.

use crate::change_joint_column_web_depth::ChangeJointColumnWebDepth;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointColumnWebDepth, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_column_web_depth::ChangeJointColumnWebDepth> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointColumnWebDepth { id: payload.id.clone(), new_column_web_depth: row.column_web_depth })],
        None => Vec::new(),
    })
}
