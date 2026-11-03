//! ↩️ `change-conn-material-id` inverse: restores the previous `material_id`.

use crate::change_conn_material_id::ChangeConnMaterialId;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeConnMaterialId, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_conn_material_id::ChangeConnMaterialId> {
    Ok(match base.connections.iter().find(|row| row.id == payload.connection_id) {
        Some(row) => vec![Op::from(ChangeConnMaterialId { connection_id: payload.connection_id.clone(), new_material_id: row.material_id.clone() })],
        None => Vec::new(),
    })
}
