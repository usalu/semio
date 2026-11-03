//! ↩️ `change-joint-building-id` inverse: restores the previous `building_id`.

use crate::change_joint_building_id::ChangeJointBuildingId;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeJointBuildingId, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_building_id::ChangeJointBuildingId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeJointBuildingId, base: &En1998Snapshot) -> Option<ChangeJointBuildingId> {
    let r0 = base.joints.iter().find(|row| row.id == payload.id)?;
    Some(ChangeJointBuildingId { id: payload.id.clone(), new_building_id: r0.building_id.clone() })
}
