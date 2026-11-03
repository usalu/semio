//! ↩️ `change-silo-depth` inverse: restores the previous `depth`.

use crate::change_silo_depth::ChangeSiloDepth;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloDepth, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_depth::ChangeSiloDepth> {
    Ok(match base.silo_shells.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSiloDepth { id: payload.id.clone(), new_depth: row.depth })],
        None => Vec::new(),
    })
}
