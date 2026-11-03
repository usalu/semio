//! ↩️ `change-support-collision-height` inverse: restores the previous `height`.

use crate::change_support_collision_height::ChangeSupportCollisionHeight;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeSupportCollisionHeight, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_support_collision_height::ChangeSupportCollisionHeight> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSupportCollisionHeight, base: &En1991Snapshot) -> Option<ChangeSupportCollisionHeight> {
    let r0 = base.support_collisions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSupportCollisionHeight { id: payload.id.clone(), new_height: r0.height })
}
