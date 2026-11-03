//! 🔺️ `insert-outline-vertex` diff.

use crate::insert_outline_vertex::InsertOutlineVertex;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999OutlineVerticesRows};

pub fn diff(payload: &InsertOutlineVertex, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if base.outline_vertices.iter().any(|existing| existing.id == payload.outline_vertex.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Outline vertex id {} already exists.", payload.outline_vertex.id), [payload.outline_vertex.id.clone()]);
    }
    if (payload.index as usize) > base.outline_vertices.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", (payload.index as usize), base.outline_vertices.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1999Diff { outline_vertices: Some(En1999OutlineVerticesRows::insertion(payload.index as usize, payload.outline_vertex.clone())), ..Default::default() })
}
