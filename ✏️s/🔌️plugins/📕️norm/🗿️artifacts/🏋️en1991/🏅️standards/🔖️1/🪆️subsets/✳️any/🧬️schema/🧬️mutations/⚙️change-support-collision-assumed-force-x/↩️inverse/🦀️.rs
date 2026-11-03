//! ↩️ `change-support-collision-assumed-force-x` inverse: restores the previous `assumed_force_x`.

use crate::change_support_collision_assumed_force_x::ChangeSupportCollisionAssumedForceX;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeSupportCollisionAssumedForceX, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_support_collision_assumed_force_x::ChangeSupportCollisionAssumedForceX> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSupportCollisionAssumedForceX, base: &En1991Snapshot) -> Option<ChangeSupportCollisionAssumedForceX> {
    let r0 = base.support_collisions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSupportCollisionAssumedForceX { id: payload.id.clone(), new_assumed_force_x: r0.assumed_force_x })
}
