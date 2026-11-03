//! ↩️ `change-emb-wall-embedment` inverse: restores the previous `embedment`.

use crate::change_emb_wall_embedment::ChangeEmbWallEmbedment;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeEmbWallEmbedment, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_emb_wall_embedment::ChangeEmbWallEmbedment> {
    Ok(match base.embedded_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeEmbWallEmbedment { id: payload.id.clone(), new_embedment: row.embedment })],
        None => Vec::new(),
    })
}
