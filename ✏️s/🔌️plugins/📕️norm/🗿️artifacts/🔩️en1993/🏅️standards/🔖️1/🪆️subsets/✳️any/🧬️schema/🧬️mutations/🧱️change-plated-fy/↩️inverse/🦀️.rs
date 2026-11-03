//! ↩️ `change-plated-fy` inverse: restores the previous `fy`.

use crate::change_plated_fy::ChangePlatedFy;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedFy, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_fy::ChangePlatedFy> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedFy { id: payload.id.clone(), new_fy: row.fy })],
        None => Vec::new(),
    })
}
