//! ↩️ `remove-construction-load` inverse: inserts the removed row back at its position.

use crate::remove_construction_load::RemoveConstructionLoad;
use crate::insert_construction_load::InsertConstructionLoad;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveConstructionLoad, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_construction_load::InsertConstructionLoad> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveConstructionLoad, base: &En1991Snapshot) -> Option<InsertConstructionLoad> {
    let (index, row) = base.construction_loads.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertConstructionLoad { index, item: row.clone() })
}
