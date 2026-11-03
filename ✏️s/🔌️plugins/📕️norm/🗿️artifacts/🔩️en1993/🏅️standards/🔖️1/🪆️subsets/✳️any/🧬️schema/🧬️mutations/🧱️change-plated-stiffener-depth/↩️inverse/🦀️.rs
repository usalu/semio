//! ↩️ `change-plated-stiffener-depth` inverse: restores the previous `stiffener_depth`.

use crate::change_plated_stiffener_depth::ChangePlatedStiffenerDepth;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedStiffenerDepth, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_stiffener_depth::ChangePlatedStiffenerDepth> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedStiffenerDepth { id: payload.id.clone(), new_stiffener_depth: row.stiffener_depth })],
        None => Vec::new(),
    })
}
