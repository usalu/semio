//! ↩️ `change-emb-wall-retained-height` inverse: restores the previous `retained_height`.

use crate::change_emb_wall_retained_height::ChangeEmbWallRetainedHeight;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeEmbWallRetainedHeight, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_emb_wall_retained_height::ChangeEmbWallRetainedHeight> {
    Ok(match base.embedded_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeEmbWallRetainedHeight { id: payload.id.clone(), new_retained_height: row.retained_height })],
        None => Vec::new(),
    })
}
