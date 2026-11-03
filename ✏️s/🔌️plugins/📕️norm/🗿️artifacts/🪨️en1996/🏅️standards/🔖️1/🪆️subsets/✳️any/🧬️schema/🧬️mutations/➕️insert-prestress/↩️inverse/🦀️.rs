//! ↩️ `insert-prestress` inverse: removes the inserted row again.

use crate::insert_prestress::InsertPrestress;
use crate::remove_prestress::RemovePrestress;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &InsertPrestress, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_prestress::RemovePrestress> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertPrestress, base: &En1996Snapshot) -> Option<RemovePrestress> {
    let _r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    Some(RemovePrestress { wall_id: payload.wall_id.clone(), id: payload.prestress.id.clone() })
}
