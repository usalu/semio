//! ↩️ `change-plated-stiffener-flange-width` inverse: restores the previous `stiffener_flange_width`.

use crate::change_plated_stiffener_flange_width::ChangePlatedStiffenerFlangeWidth;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedStiffenerFlangeWidth, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_stiffener_flange_width::ChangePlatedStiffenerFlangeWidth> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedStiffenerFlangeWidth { id: payload.id.clone(), new_stiffener_flange_width: row.stiffener_flange_width })],
        None => Vec::new(),
    })
}
