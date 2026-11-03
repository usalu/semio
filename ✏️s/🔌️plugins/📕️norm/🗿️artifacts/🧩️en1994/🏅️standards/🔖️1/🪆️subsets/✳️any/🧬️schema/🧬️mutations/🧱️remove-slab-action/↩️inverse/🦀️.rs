//! ↩️ `remove-slab-action` inverse: inserts the removed row back at its position.

use crate::remove_slab_action::RemoveSlabAction;
use crate::insert_slab_action::InsertSlabAction;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

pub fn inverse<Op>(payload: &RemoveSlabAction, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_slab_action::InsertSlabAction> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveSlabAction, base: &En1994Snapshot) -> Option<InsertSlabAction> {
    let r0 = base.slabs.iter().find(|row| row.id == payload.slab_id)?;
    let (index, row) = r0.actions.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertSlabAction { slab_id: payload.slab_id.clone(), index, action: row.clone() })
}
