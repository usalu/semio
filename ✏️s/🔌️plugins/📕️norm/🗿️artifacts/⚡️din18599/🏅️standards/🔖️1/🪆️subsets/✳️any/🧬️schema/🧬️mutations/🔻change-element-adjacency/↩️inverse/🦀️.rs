//! ↩️ `change-element-adjacency` inverse.

use crate::change_element_adjacency::ChangeElementAdjacency;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &ChangeElementAdjacency, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_element_adjacency::ChangeElementAdjacency> {
    Ok(base.elements.iter().find(|row| row.id == payload.element_id).map(|row| vec![Op::from(ChangeElementAdjacency { element_id: payload.element_id.clone(), new_adjacency: row.adjacency })]).unwrap_or_default())
}
