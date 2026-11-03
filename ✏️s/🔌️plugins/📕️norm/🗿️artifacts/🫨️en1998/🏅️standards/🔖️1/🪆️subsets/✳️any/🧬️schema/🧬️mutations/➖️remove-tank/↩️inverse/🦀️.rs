//! ↩️ `remove-tank` inverse: inserts the removed row back at its position.

use crate::remove_tank::RemoveTank;
use crate::insert_tank::InsertTank;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &RemoveTank, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_tank::InsertTank> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveTank, base: &En1998Snapshot) -> Option<InsertTank> {
    let (index, row) = base.tanks.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertTank { index: Some(index), tank: row.clone() })
}
