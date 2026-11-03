//! 🔺️ `change-vertex-z` diff: patches `z` of the addressed outline vertex row.

use crate::change_vertex_z::ChangeVertexZ;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999OutlineVerticesPatch, En1999OutlineVerticesRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeVertexZ, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_z.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.outline_vertices.iter().find(|row| row.id == payload.outline_vertex_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "outline-vertex", Vec::<String>::new());
    };
    if r0.z == payload.new_z {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The outline vertex already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { outline_vertices: Some(En1999OutlineVerticesRows::modification(&payload.outline_vertex_id, En1999OutlineVerticesPatch { z: Some(payload.new_z), ..Default::default() })), ..Default::default() })
}
