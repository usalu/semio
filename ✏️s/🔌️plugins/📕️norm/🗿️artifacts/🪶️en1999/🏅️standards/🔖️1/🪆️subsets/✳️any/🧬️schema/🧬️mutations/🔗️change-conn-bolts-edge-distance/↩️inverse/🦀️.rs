//! ↩️ `change-conn-bolts-edge-distance` inverse: restores the previous `bolts.edge_distance`.

use crate::change_conn_bolts_edge_distance::ChangeConnBoltsEdgeDistance;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeConnBoltsEdgeDistance, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_conn_bolts_edge_distance::ChangeConnBoltsEdgeDistance> {
    Ok(match base.connections.iter().find(|row| row.id == payload.connection_id) {
        Some(row) => vec![Op::from(ChangeConnBoltsEdgeDistance { connection_id: payload.connection_id.clone(), new_bolts_edge_distance: row.bolts.edge_distance })],
        None => Vec::new(),
    })
}
