//! ↩️ `change-conn-welds-filler-alloy` inverse: restores the previous `welds.filler_alloy`.

use crate::change_conn_welds_filler_alloy::ChangeConnWeldsFillerAlloy;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeConnWeldsFillerAlloy, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_conn_welds_filler_alloy::ChangeConnWeldsFillerAlloy> {
    Ok(match base.connections.iter().find(|row| row.id == payload.connection_id) {
        Some(row) => vec![Op::from(ChangeConnWeldsFillerAlloy { connection_id: payload.connection_id.clone(), new_welds_filler_alloy: row.welds.filler_alloy.clone() })],
        None => Vec::new(),
    })
}
