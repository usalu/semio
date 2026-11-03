//! ↩️ `change-conn-welds-haz-extent` inverse: restores the previous `welds.haz_extent`.

use crate::change_conn_welds_haz_extent::ChangeConnWeldsHazExtent;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeConnWeldsHazExtent, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_conn_welds_haz_extent::ChangeConnWeldsHazExtent> {
    Ok(match base.connections.iter().find(|row| row.id == payload.connection_id) {
        Some(row) => vec![Op::from(ChangeConnWeldsHazExtent { connection_id: payload.connection_id.clone(), new_welds_haz_extent: row.welds.haz_extent })],
        None => Vec::new(),
    })
}
