//! 🔺️ `change-vertex-ring` diff: patches `ring` of the addressed outline vertex row.

use crate::change_vertex_ring::ChangeVertexRing;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999OutlineVerticesPatch, En1999OutlineVerticesRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeVertexRing, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(r0) = base.outline_vertices.iter().find(|row| row.id == payload.outline_vertex_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "outline-vertex", Vec::<String>::new());
    };
    if r0.ring == payload.new_ring {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The outline vertex already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { outline_vertices: Some(En1999OutlineVerticesRows::modification(&payload.outline_vertex_id, En1999OutlineVerticesPatch { ring: Some(payload.new_ring), ..Default::default() })), ..Default::default() })
}
