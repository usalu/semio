//! ↩️ `change-vertex-section-id` inverse: restores the previous `section_id`.

use crate::change_vertex_section_id::ChangeVertexSectionId;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeVertexSectionId, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_vertex_section_id::ChangeVertexSectionId> {
    Ok(match base.outline_vertices.iter().find(|row| row.id == payload.outline_vertex_id) {
        Some(row) => vec![Op::from(ChangeVertexSectionId { outline_vertex_id: payload.outline_vertex_id.clone(), new_section_id: row.section_id.clone() })],
        None => Vec::new(),
    })
}
