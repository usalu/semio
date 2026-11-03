//! ↩️ `change-support-collision-assumed-force-y` inverse: restores the previous `assumed_force_y`.

use crate::change_support_collision_assumed_force_y::ChangeSupportCollisionAssumedForceY;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeSupportCollisionAssumedForceY, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_support_collision_assumed_force_y::ChangeSupportCollisionAssumedForceY> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSupportCollisionAssumedForceY, base: &En1991Snapshot) -> Option<ChangeSupportCollisionAssumedForceY> {
    let r0 = base.support_collisions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSupportCollisionAssumedForceY { id: payload.id.clone(), new_assumed_force_y: r0.assumed_force_y })
}
