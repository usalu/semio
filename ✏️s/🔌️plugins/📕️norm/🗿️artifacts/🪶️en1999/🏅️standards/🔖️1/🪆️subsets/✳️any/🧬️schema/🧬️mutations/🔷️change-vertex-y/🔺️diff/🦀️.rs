//! 🔺️ `change-vertex-y` diff: patches `y` of the addressed outline vertex row.

use crate::change_vertex_y::ChangeVertexY;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999OutlineVerticesPatch, En1999OutlineVerticesRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeVertexY, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.outline_vertices.iter().find(|row| row.id == payload.outline_vertex_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "outline-vertex", Vec::<String>::new());
    };
    if r0.y == payload.new_y {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The outline vertex already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { outline_vertices: Some(En1999OutlineVerticesRows::modification(&payload.outline_vertex_id, En1999OutlineVerticesPatch { y: Some(payload.new_y), ..Default::default() })), ..Default::default() })
}
