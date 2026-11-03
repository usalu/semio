//! ↩️ `change-joint-storey-id` inverse: restores the previous `storey_id`.

use crate::change_joint_storey_id::ChangeJointStoreyId;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeJointStoreyId, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_storey_id::ChangeJointStoreyId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeJointStoreyId, base: &En1998Snapshot) -> Option<ChangeJointStoreyId> {
    let r0 = base.joints.iter().find(|row| row.id == payload.id)?;
    Some(ChangeJointStoreyId { id: payload.id.clone(), new_storey_id: r0.storey_id.clone() })
}
