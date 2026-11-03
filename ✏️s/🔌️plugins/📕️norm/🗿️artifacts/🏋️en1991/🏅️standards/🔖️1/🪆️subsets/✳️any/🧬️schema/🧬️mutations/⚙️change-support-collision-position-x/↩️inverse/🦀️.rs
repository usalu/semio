//! ↩️ `change-support-collision-position-x` inverse: restores the previous `position_x`.

use crate::change_support_collision_position_x::ChangeSupportCollisionPositionX;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeSupportCollisionPositionX, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_support_collision_position_x::ChangeSupportCollisionPositionX> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSupportCollisionPositionX, base: &En1991Snapshot) -> Option<ChangeSupportCollisionPositionX> {
    let r0 = base.support_collisions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSupportCollisionPositionX { id: payload.id.clone(), new_position_x: r0.position_x })
}
