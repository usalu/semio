//! ↩️ `change-vertex-element-id` inverse: restores the previous `element_id`.

use crate::change_vertex_element_id::ChangeVertexElementId;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeVertexElementId, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_vertex_element_id::ChangeVertexElementId> {
    Ok(match base.outline_vertices.iter().find(|row| row.id == payload.outline_vertex_id) {
        Some(row) => vec![Op::from(ChangeVertexElementId { outline_vertex_id: payload.outline_vertex_id.clone(), new_element_id: row.element_id.clone() })],
        None => Vec::new(),
    })
}
