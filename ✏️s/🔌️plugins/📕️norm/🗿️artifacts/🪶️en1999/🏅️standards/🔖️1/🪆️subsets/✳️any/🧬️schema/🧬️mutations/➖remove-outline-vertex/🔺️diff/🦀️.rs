//! 🔺️ `remove-outline-vertex` diff.

use crate::remove_outline_vertex::RemoveOutlineVertex;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999OutlineVerticesRows};

pub fn diff(payload: &RemoveOutlineVertex, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(index) = base.outline_vertices.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown outline vertex id {}", payload.id), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1999Diff { outline_vertices: Some(En1999OutlineVerticesRows::removal(&base.outline_vertices, index)), ..Default::default() })
}
