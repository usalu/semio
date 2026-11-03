//! ↩️ `insert-bar-layer` inverse: removes the inserted row again.

use crate::insert_bar_layer::InsertBarLayer;
use crate::remove_bar_layer::RemoveBarLayer;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &InsertBarLayer, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_bar_layer::RemoveBarLayer> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertBarLayer, base: &En1992Snapshot) -> Option<RemoveBarLayer> {
    let _r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let index = payload.index;
    Some(RemoveBarLayer { member_id: payload.member_id.clone(), index })
}
