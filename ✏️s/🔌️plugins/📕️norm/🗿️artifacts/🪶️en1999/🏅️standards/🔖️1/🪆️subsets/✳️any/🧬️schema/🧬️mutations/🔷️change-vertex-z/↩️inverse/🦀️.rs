//! ↩️ `change-vertex-z` inverse: restores the previous `z`.

use crate::change_vertex_z::ChangeVertexZ;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeVertexZ, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_vertex_z::ChangeVertexZ> {
    Ok(match base.outline_vertices.iter().find(|row| row.id == payload.outline_vertex_id) {
        Some(row) => vec![Op::from(ChangeVertexZ { outline_vertex_id: payload.outline_vertex_id.clone(), new_z: row.z })],
        None => Vec::new(),
    })
}
