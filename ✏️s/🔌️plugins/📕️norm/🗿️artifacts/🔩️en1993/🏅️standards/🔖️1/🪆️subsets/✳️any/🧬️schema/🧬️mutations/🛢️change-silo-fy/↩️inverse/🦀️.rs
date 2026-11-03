//! ↩️ `change-silo-fy` inverse: restores the previous `fy`.

use crate::change_silo_fy::ChangeSiloFy;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloFy, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_fy::ChangeSiloFy> {
    Ok(match base.silo_shells.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSiloFy { id: payload.id.clone(), new_fy: row.fy })],
        None => Vec::new(),
    })
}
