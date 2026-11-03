//! ↩️ `change-transverse-width` inverse: restores the previous `width`.

use crate::change_transverse_width::ChangeTransverseWidth;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeTransverseWidth, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_transverse_width::ChangeTransverseWidth> {
    Ok(match base.transverse_stiffeners.iter().find(|row| row.id == payload.transverse_stiffener_id) {
        Some(row) => vec![Op::from(ChangeTransverseWidth { transverse_stiffener_id: payload.transverse_stiffener_id.clone(), new_width: row.width })],
        None => Vec::new(),
    })
}
