//! ↩️ `insert-support-collision` inverse: removes the inserted row again.

use crate::insert_support_collision::InsertSupportCollision;
use crate::remove_support_collision::RemoveSupportCollision;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertSupportCollision, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_support_collision::RemoveSupportCollision> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertSupportCollision, base: &En1991Snapshot) -> Option<RemoveSupportCollision> {
    Some(RemoveSupportCollision { id: payload.item.id.clone() })
}
