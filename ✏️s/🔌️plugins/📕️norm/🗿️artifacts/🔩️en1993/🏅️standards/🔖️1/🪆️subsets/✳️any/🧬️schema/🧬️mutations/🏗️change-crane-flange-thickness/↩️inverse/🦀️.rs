//! ↩️ `change-crane-flange-thickness` inverse: restores the previous `flange_thickness`.

use crate::change_crane_flange_thickness::ChangeCraneFlangeThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCraneFlangeThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_flange_thickness::ChangeCraneFlangeThickness> {
    Ok(match base.crane_runways.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCraneFlangeThickness { id: payload.id.clone(), new_flange_thickness: row.flange_thickness })],
        None => Vec::new(),
    })
}
