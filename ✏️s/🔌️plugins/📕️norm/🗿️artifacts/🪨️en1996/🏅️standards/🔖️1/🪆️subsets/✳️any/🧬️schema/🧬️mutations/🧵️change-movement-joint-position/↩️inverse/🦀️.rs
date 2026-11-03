//! ↩️ `change-movement-joint-position` inverse: restores the previous `position_m`.

use crate::change_movement_joint_position::ChangeMovementJointPosition;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeMovementJointPosition, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_movement_joint_position::ChangeMovementJointPosition> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMovementJointPosition, base: &En1996Snapshot) -> Option<ChangeMovementJointPosition> {
    let r0 = base.movement_joints.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMovementJointPosition { id: payload.id.clone(), new_position_m: r0.position_m })
}
