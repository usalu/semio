//! ↩️ `remove-bar-layer` inverse: inserts the removed row back at its position.

use crate::remove_bar_layer::RemoveBarLayer;
use crate::insert_bar_layer::InsertBarLayer;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &RemoveBarLayer, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_bar_layer::InsertBarLayer> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveBarLayer, base: &En1992Snapshot) -> Option<InsertBarLayer> {
    let r0 = base.members.iter().find(|row| row.id == payload.member_id)?;
    let index = payload.index;
    let row = r0.longitudinal.get(index)?;
    Some(InsertBarLayer { member_id: payload.member_id.clone(), index, bar_layer: row.clone() })
}
