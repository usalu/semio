//! ↩️ `change-joint-category` inverse: restores the previous `category`.

use crate::change_joint_category::ChangeJointCategory;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointCategory, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_category::ChangeJointCategory> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointCategory { id: payload.id.clone(), new_category: row.category })],
        None => Vec::new(),
    })
}
