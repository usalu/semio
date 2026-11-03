//! ↩️ `insert-outline-vertex` inverse.

use crate::insert_outline_vertex::InsertOutlineVertex;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::remove_outline_vertex;

pub fn inverse<Op>(payload: &InsertOutlineVertex, _base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_outline_vertex::RemoveOutlineVertex> {
    Ok((|| {
    vec![Op::from(remove_outline_vertex::RemoveOutlineVertex { id: payload.outline_vertex.id.clone() })]

    })())
}
