//! ↩️ `change-emb-wall-prop-depth` inverse: restores the previous `prop_depth`.

use crate::change_emb_wall_prop_depth::ChangeEmbWallPropDepth;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeEmbWallPropDepth, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_emb_wall_prop_depth::ChangeEmbWallPropDepth> {
    Ok(match base.embedded_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeEmbWallPropDepth { id: payload.id.clone(), new_prop_depth: row.prop_depth })],
        None => Vec::new(),
    })
}
