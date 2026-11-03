//! ↩️ `change-wall-ocr` inverse: restores the previous `ocr`.

use crate::change_wall_ocr::ChangeWallOcr;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeWallOcr, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_ocr::ChangeWallOcr> {
    Ok(match base.retaining_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWallOcr { id: payload.id.clone(), new_ocr: row.ocr })],
        None => Vec::new(),
    })
}
