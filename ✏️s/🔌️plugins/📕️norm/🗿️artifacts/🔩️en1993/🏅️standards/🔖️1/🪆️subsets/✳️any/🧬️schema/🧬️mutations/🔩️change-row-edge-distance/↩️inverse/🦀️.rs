//! ↩️ `change-row-edge-distance` inverse: restores the previous `edge_distance`.

use crate::change_row_edge_distance::ChangeRowEdgeDistance;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRowEdgeDistance, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_row_edge_distance::ChangeRowEdgeDistance> {
    Ok(match base.joint_rows.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRowEdgeDistance { id: payload.id.clone(), new_edge_distance: row.edge_distance })],
        None => Vec::new(),
    })
}
