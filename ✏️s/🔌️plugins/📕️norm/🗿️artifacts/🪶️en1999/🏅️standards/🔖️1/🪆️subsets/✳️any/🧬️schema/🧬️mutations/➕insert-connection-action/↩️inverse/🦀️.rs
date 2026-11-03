//! ↩️ `insert-connection-action` inverse.

use crate::insert_connection_action::InsertConnectionAction;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::remove_connection_action;

pub fn inverse<Op>(payload: &InsertConnectionAction, _base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_connection_action::RemoveConnectionAction> {
    Ok((|| {
    vec![Op::from(remove_connection_action::RemoveConnectionAction { connection_id: payload.connection_id.clone(), id: payload.action.id.clone() })]
    })())
}
