//! ↩️ `change-conn-load-eccentricity` inverse: restores the previous `load_eccentricity`.

use crate::change_conn_load_eccentricity::ChangeConnLoadEccentricity;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeConnLoadEccentricity, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_conn_load_eccentricity::ChangeConnLoadEccentricity> {
    Ok(match base.connections.iter().find(|row| row.id == payload.aluminium_connection_id) {
        Some(row) => vec![Op::from(ChangeConnLoadEccentricity { aluminium_connection_id: payload.aluminium_connection_id.clone(), new_load_eccentricity: row.load_eccentricity })],
        None => Vec::new(),
    })
}
