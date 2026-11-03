//! ↩️ `change-wall-type` inverse: restores the previous `wall_type`.

use crate::change_wall_type::ChangeWallType;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeWallType, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_type::ChangeWallType> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeWallType, base: &En1996Snapshot) -> Option<ChangeWallType> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeWallType { id: payload.id.clone(), new_wall_type: r0.wall_type.clone() })
}
