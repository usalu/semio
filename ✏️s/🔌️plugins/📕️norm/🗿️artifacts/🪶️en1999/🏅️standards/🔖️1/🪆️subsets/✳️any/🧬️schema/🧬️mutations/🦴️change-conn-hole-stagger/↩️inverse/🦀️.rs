//! ↩️ `change-conn-hole-stagger` inverse: restores the previous `hole_stagger`.

use crate::change_conn_hole_stagger::ChangeConnHoleStagger;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeConnHoleStagger, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_conn_hole_stagger::ChangeConnHoleStagger> {
    Ok(match base.connections.iter().find(|row| row.id == payload.aluminium_connection_id) {
        Some(row) => vec![Op::from(ChangeConnHoleStagger { aluminium_connection_id: payload.aluminium_connection_id.clone(), new_hole_stagger: row.hole_stagger })],
        None => Vec::new(),
    })
}
