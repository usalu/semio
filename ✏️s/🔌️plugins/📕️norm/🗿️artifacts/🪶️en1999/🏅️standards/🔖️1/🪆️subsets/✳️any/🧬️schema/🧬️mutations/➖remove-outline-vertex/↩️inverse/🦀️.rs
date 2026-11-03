//! ↩️ `remove-outline-vertex` inverse.

use crate::remove_outline_vertex::RemoveOutlineVertex;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::insert_outline_vertex;

pub fn inverse<Op>(payload: &RemoveOutlineVertex, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_outline_vertex::InsertOutlineVertex> {
    Ok((|| {
    if let Some((index, row)) = base.outline_vertices.iter().enumerate().find(|(_, row)| row.id == payload.id) {
        vec![Op::from(insert_outline_vertex::InsertOutlineVertex { index: index as u32, outline_vertex: row.clone() })]
    } else { Vec::new() }

    })())
}
