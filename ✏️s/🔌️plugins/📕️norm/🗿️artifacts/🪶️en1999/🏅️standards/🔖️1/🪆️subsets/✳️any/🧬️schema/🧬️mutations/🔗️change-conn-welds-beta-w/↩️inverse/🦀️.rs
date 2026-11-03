//! ↩️ `change-conn-welds-beta-w` inverse: restores the previous `welds.beta_w`.

use crate::change_conn_welds_beta_w::ChangeConnWeldsBetaW;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeConnWeldsBetaW, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_conn_welds_beta_w::ChangeConnWeldsBetaW> {
    Ok(match base.connections.iter().find(|row| row.id == payload.connection_id) {
        Some(row) => vec![Op::from(ChangeConnWeldsBetaW { connection_id: payload.connection_id.clone(), new_welds_beta_w: row.welds.beta_w })],
        None => Vec::new(),
    })
}
