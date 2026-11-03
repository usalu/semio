//! ↩️ `change-wall-embedment` inverse: restores the previous `embedment`.

use crate::change_wall_embedment::ChangeWallEmbedment;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeWallEmbedment, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_embedment::ChangeWallEmbedment> {
    Ok(match base.retaining_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWallEmbedment { id: payload.id.clone(), new_embedment: row.embedment })],
        None => Vec::new(),
    })
}
