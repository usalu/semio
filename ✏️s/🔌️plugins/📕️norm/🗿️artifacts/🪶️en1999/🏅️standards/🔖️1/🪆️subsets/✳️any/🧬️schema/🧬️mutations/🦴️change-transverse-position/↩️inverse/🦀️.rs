//! ↩️ `change-transverse-position` inverse: restores the previous `position`.

use crate::change_transverse_position::ChangeTransversePosition;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeTransversePosition, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_transverse_position::ChangeTransversePosition> {
    Ok(match base.transverse_stiffeners.iter().find(|row| row.id == payload.transverse_stiffener_id) {
        Some(row) => vec![Op::from(ChangeTransversePosition { transverse_stiffener_id: payload.transverse_stiffener_id.clone(), new_position: row.position })],
        None => Vec::new(),
    })
}
