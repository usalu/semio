//! ↩️ `change-vertex-y` inverse: restores the previous `y`.

use crate::change_vertex_y::ChangeVertexY;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeVertexY, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_vertex_y::ChangeVertexY> {
    Ok(match base.outline_vertices.iter().find(|row| row.id == payload.outline_vertex_id) {
        Some(row) => vec![Op::from(ChangeVertexY { outline_vertex_id: payload.outline_vertex_id.clone(), new_y: row.y })],
        None => Vec::new(),
    })
}
