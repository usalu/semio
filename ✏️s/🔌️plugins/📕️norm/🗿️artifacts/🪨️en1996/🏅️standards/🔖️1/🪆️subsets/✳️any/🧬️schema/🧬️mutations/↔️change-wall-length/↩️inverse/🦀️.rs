//! ↩️ `change-wall-length` inverse: restores the previous `length_m`.

use crate::change_wall_length::ChangeWallLength;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeWallLength, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_length::ChangeWallLength> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeWallLength, base: &En1996Snapshot) -> Option<ChangeWallLength> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeWallLength { id: payload.id.clone(), new_length_m: r0.length_m })
}
