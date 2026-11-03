//! ↩️ `change-plated-stiff-flange-width` inverse: restores the previous `stiffener_flange_width`.

use crate::change_plated_stiff_flange_width::ChangePlatedStiffFlangeWidth;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedStiffFlangeWidth, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_stiff_flange_width::ChangePlatedStiffFlangeWidth> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedStiffFlangeWidth { id: payload.id.clone(), new_stiffener_flange_width: row.stiffener_flange_width })],
        None => Vec::new(),
    })
}
