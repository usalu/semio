//! ↩️ `change-support-collision-location` inverse: restores the previous `location`.

use crate::change_support_collision_location::ChangeSupportCollisionLocation;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeSupportCollisionLocation, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_support_collision_location::ChangeSupportCollisionLocation> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSupportCollisionLocation, base: &En1991Snapshot) -> Option<ChangeSupportCollisionLocation> {
    let r0 = base.support_collisions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSupportCollisionLocation { id: payload.id.clone(), new_location: r0.location.clone() })
}
