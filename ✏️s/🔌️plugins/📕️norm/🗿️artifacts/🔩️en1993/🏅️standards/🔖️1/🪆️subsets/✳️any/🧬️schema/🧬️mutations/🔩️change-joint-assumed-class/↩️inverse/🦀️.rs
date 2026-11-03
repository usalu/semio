//! ↩️ `change-joint-assumed-class` inverse: restores the previous `assumed_class`.

use crate::change_joint_assumed_class::ChangeJointAssumedClass;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointAssumedClass, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_assumed_class::ChangeJointAssumedClass> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointAssumedClass { id: payload.id.clone(), new_assumed_class: row.assumed_class })],
        None => Vec::new(),
    })
}
