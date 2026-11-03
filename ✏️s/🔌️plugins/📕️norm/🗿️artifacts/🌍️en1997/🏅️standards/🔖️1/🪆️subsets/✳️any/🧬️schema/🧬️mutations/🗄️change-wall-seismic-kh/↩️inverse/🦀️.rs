//! ↩️ `change-wall-seismic-kh` inverse: restores the previous `seismic_kh`.

use crate::change_wall_seismic_kh::ChangeWallSeismicKh;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeWallSeismicKh, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_wall_seismic_kh::ChangeWallSeismicKh> {
    Ok(match base.retaining_walls.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWallSeismicKh { id: payload.id.clone(), new_seismic_kh: row.seismic_kh })],
        None => Vec::new(),
    })
}
