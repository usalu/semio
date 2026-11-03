//! ↩️ `change-wall-height` inverse: restores the previous `height_m`.

use crate::change_wall_height::ChangeWallHeight;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeWallHeight, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_height::ChangeWallHeight> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeWallHeight, base: &En1996Snapshot) -> Option<ChangeWallHeight> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeWallHeight { id: payload.id.clone(), new_height_m: r0.height_m })
}
