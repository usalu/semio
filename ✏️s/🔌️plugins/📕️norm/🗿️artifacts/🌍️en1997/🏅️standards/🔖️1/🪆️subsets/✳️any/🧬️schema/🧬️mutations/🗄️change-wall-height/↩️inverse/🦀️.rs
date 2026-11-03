//! ↩️ `change-wall-height` inverse: restores the previous `height`.

use crate::change_wall_height::ChangeWallHeight;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeWallHeight, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_height::ChangeWallHeight> {
    Ok(match base.retaining_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWallHeight { id: payload.id.clone(), new_height: row.height })],
        None => Vec::new(),
    })
}
