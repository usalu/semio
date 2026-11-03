//! ↩️ `change-wall-stem-thickness` inverse: restores the previous `stem_thickness`.

use crate::change_wall_stem_thickness::ChangeWallStemThickness;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeWallStemThickness, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_stem_thickness::ChangeWallStemThickness> {
    Ok(match base.retaining_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWallStemThickness { id: payload.id.clone(), new_stem_thickness: row.stem_thickness })],
        None => Vec::new(),
    })
}
