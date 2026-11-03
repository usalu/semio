//! ↩️ `change-transverse-girder-id` inverse: restores the previous `girder_id`.

use crate::change_transverse_girder_id::ChangeTransverseGirderId;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeTransverseGirderId, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_transverse_girder_id::ChangeTransverseGirderId> {
    Ok(match base.transverse_stiffeners.iter().find(|row| row.id == payload.transverse_stiffener_id) {
        Some(row) => vec![Op::from(ChangeTransverseGirderId { transverse_stiffener_id: payload.transverse_stiffener_id.clone(), new_girder_id: row.girder_id.clone() })],
        None => Vec::new(),
    })
}
