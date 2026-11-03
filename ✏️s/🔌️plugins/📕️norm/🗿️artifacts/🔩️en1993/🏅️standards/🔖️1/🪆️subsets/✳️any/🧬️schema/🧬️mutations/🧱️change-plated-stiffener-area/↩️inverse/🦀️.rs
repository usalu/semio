//! ↩️ `change-plated-stiffener-area` inverse: restores the previous `stiffener_area`.

use crate::change_plated_stiffener_area::ChangePlatedStiffenerArea;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedStiffenerArea, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_stiffener_area::ChangePlatedStiffenerArea> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedStiffenerArea { id: payload.id.clone(), new_stiffener_area: row.stiffener_area })],
        None => Vec::new(),
    })
}
