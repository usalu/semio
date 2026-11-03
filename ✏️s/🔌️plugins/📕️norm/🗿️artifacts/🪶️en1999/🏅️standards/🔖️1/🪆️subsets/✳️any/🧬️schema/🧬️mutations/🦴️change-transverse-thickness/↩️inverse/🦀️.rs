//! ↩️ `change-transverse-thickness` inverse: restores the previous `thickness`.

use crate::change_transverse_thickness::ChangeTransverseThickness;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeTransverseThickness, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_transverse_thickness::ChangeTransverseThickness> {
    Ok(match base.transverse_stiffeners.iter().find(|row| row.id == payload.transverse_stiffener_id) {
        Some(row) => vec![Op::from(ChangeTransverseThickness { transverse_stiffener_id: payload.transverse_stiffener_id.clone(), new_thickness: row.thickness })],
        None => Vec::new(),
    })
}
