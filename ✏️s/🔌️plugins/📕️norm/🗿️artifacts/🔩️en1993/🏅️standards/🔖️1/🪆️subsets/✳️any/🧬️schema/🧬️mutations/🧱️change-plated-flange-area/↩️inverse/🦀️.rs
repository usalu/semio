//! ↩️ `change-plated-flange-area` inverse: restores the previous `flange_area`.

use crate::change_plated_flange_area::ChangePlatedFlangeArea;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedFlangeArea, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_flange_area::ChangePlatedFlangeArea> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedFlangeArea { id: payload.id.clone(), new_flange_area: row.flange_area })],
        None => Vec::new(),
    })
}
