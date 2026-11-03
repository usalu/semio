//! ↩️ `change-conn-action-category` inverse: restores the previous `category`.

use crate::change_conn_action_category::ChangeConnActionCategory;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeConnActionCategory, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_conn_action_category::ChangeConnActionCategory> {
    Ok(match base.connections.iter().find(|row| row.id == payload.connection_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeConnActionCategory { connection_id: payload.connection_id.clone(), action_id: payload.action_id.clone(), new_category: row.category })],
        None => Vec::new(),
    })
}
