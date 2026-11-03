//! ↩️ `insert-movement-joint` inverse: removes the inserted row again.

use crate::insert_movement_joint::InsertMovementJoint;
use crate::remove_movement_joint::RemoveMovementJoint;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &InsertMovementJoint, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_movement_joint::RemoveMovementJoint> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertMovementJoint, base: &En1996Snapshot) -> Option<RemoveMovementJoint> {
    Some(RemoveMovementJoint { id: payload.item.id.clone() })
}
