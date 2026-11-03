//! ↩️ `remove-footbridge` inverse: inserts the removed row back at its position.

use crate::remove_footbridge::RemoveFootbridge;
use crate::insert_footbridge::InsertFootbridge;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveFootbridge, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_footbridge::InsertFootbridge> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveFootbridge, base: &En1991Snapshot) -> Option<InsertFootbridge> {
    let (index, row) = base.footbridges.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertFootbridge { index, item: row.clone() })
}
