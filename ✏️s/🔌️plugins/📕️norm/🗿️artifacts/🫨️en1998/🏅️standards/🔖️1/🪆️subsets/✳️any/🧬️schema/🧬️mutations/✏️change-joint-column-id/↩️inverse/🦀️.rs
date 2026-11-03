//! ↩️ `change-joint-column-id` inverse: restores the previous `column_id`.

use crate::change_joint_column_id::ChangeJointColumnId;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeJointColumnId, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_column_id::ChangeJointColumnId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeJointColumnId, base: &En1998Snapshot) -> Option<ChangeJointColumnId> {
    let r0 = base.joints.iter().find(|row| row.id == payload.id)?;
    Some(ChangeJointColumnId { id: payload.id.clone(), new_column_id: r0.column_id.clone() })
}
