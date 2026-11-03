//! ↩️ `change-plated-flange-width` inverse: restores the previous `flange_width`.

use crate::change_plated_flange_width::ChangePlatedFlangeWidth;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedFlangeWidth, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_flange_width::ChangePlatedFlangeWidth> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedFlangeWidth { id: payload.id.clone(), new_flange_width: row.flange_width })],
        None => Vec::new(),
    })
}
