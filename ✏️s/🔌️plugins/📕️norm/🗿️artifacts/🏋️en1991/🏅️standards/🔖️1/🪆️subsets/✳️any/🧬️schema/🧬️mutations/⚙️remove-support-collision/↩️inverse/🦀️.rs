//! ↩️ `remove-support-collision` inverse: inserts the removed row back at its position.

use crate::remove_support_collision::RemoveSupportCollision;
use crate::insert_support_collision::InsertSupportCollision;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveSupportCollision, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_support_collision::InsertSupportCollision> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveSupportCollision, base: &En1991Snapshot) -> Option<InsertSupportCollision> {
    let (index, row) = base.support_collisions.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertSupportCollision { index, item: row.clone() })
}
