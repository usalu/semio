//! ↩️ `remove-helicopter` inverse: inserts the removed row back at its position.

use crate::remove_helicopter::RemoveHelicopter;
use crate::insert_helicopter::InsertHelicopter;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveHelicopter, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_helicopter::InsertHelicopter> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveHelicopter, base: &En1991Snapshot) -> Option<InsertHelicopter> {
    let (index, row) = base.helicopter_landings.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertHelicopter { index, item: row.clone() })
}
