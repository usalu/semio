//! ↩️ `change-emb-wall-thickness` inverse: restores the previous `thickness`.

use crate::change_emb_wall_thickness::ChangeEmbWallThickness;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeEmbWallThickness, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_emb_wall_thickness::ChangeEmbWallThickness> {
    Ok(match base.embedded_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeEmbWallThickness { id: payload.id.clone(), new_thickness: row.thickness })],
        None => Vec::new(),
    })
}
