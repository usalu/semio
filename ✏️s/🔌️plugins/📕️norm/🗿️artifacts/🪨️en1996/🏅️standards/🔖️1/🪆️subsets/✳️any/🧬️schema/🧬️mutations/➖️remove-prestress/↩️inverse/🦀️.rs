//! ↩️ `remove-prestress` inverse: inserts the removed row back at its position.

use crate::remove_prestress::RemovePrestress;
use crate::insert_prestress::InsertPrestress;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &RemovePrestress, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_prestress::InsertPrestress> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemovePrestress, base: &En1996Snapshot) -> Option<InsertPrestress> {
    let r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    let (index, row) = r0.prestress.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertPrestress { wall_id: payload.wall_id.clone(), index, prestress: row.clone() })
}
