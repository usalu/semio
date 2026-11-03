//! ↩️ `insert-slab-action` inverse: removes the inserted row again.

use crate::insert_slab_action::InsertSlabAction;
use crate::remove_slab_action::RemoveSlabAction;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

pub fn inverse<Op>(payload: &InsertSlabAction, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_slab_action::RemoveSlabAction> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertSlabAction, base: &En1994Snapshot) -> Option<RemoveSlabAction> {
    let _r0 = base.slabs.iter().find(|row| row.id == payload.slab_id)?;
    Some(RemoveSlabAction { slab_id: payload.slab_id.clone(), id: payload.action.id.clone() })
}
