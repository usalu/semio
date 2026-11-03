//! ↩️ `insert-tie` inverse: removes the inserted row again.

use crate::insert_tie::InsertTie;
use crate::remove_tie::RemoveTie;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &InsertTie, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_tie::RemoveTie> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertTie, base: &En1996Snapshot) -> Option<RemoveTie> {
    let _r0 = base.walls.iter().find(|row| row.id == payload.wall_id)?;
    Some(RemoveTie { wall_id: payload.wall_id.clone(), id: payload.tie.id.clone() })
}
