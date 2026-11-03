//! ↩️ `change-emb-wall-kind` inverse: restores the previous `kind`.

use crate::change_emb_wall_kind::ChangeEmbWallKind;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeEmbWallKind, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_emb_wall_kind::ChangeEmbWallKind> {
    Ok(match base.embedded_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeEmbWallKind { id: payload.id.clone(), new_kind: row.kind })],
        None => Vec::new(),
    })
}
