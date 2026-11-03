//! ↩️ `change-plated-flange-fy` inverse: restores the previous `flange_fy`.

use crate::change_plated_flange_fy::ChangePlatedFlangeFy;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedFlangeFy, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_flange_fy::ChangePlatedFlangeFy> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedFlangeFy { id: payload.id.clone(), new_flange_fy: row.flange_fy })],
        None => Vec::new(),
    })
}
