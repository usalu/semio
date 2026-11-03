//! ↩️ `remove-connection-action` inverse.

use crate::remove_connection_action::RemoveConnectionAction;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::insert_connection_action;

pub fn inverse<Op>(payload: &RemoveConnectionAction, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_connection_action::InsertConnectionAction> {
    Ok((|| {
    match base.connections.iter().find(|row| row.id == payload.connection_id).and_then(|parent| parent.actions.iter().enumerate().find(|(_, row)| row.id == payload.id)) {
        Some((index, row)) => vec![Op::from(insert_connection_action::InsertConnectionAction { connection_id: payload.connection_id.clone(), index: index as u32, action: row.clone() })],
        None => Vec::new(),
    }
    })())
}
