//! ↩️ `change-vertex-ring` inverse: restores the previous `ring`.

use crate::change_vertex_ring::ChangeVertexRing;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeVertexRing, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_vertex_ring::ChangeVertexRing> {
    Ok(match base.outline_vertices.iter().find(|row| row.id == payload.outline_vertex_id) {
        Some(row) => vec![Op::from(ChangeVertexRing { outline_vertex_id: payload.outline_vertex_id.clone(), new_ring: row.ring })],
        None => Vec::new(),
    })
}
