//! ↩️ `remove-movement-joint` inverse: inserts the removed row back at its position.

use crate::remove_movement_joint::RemoveMovementJoint;
use crate::insert_movement_joint::InsertMovementJoint;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &RemoveMovementJoint, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_movement_joint::InsertMovementJoint> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveMovementJoint, base: &En1996Snapshot) -> Option<InsertMovementJoint> {
    let (index, row) = base.movement_joints.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertMovementJoint { index, item: row.clone() })
}
