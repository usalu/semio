//! ↩️ `change-movement-joint-wall` inverse: restores the previous `wall_id`.

use crate::change_movement_joint_wall::ChangeMovementJointWall;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeMovementJointWall, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_movement_joint_wall::ChangeMovementJointWall> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeMovementJointWall, base: &En1996Snapshot) -> Option<ChangeMovementJointWall> {
    let r0 = base.movement_joints.iter().find(|row| row.id == payload.id)?;
    Some(ChangeMovementJointWall { id: payload.id.clone(), new_wall_id: r0.wall_id.clone() })
}
