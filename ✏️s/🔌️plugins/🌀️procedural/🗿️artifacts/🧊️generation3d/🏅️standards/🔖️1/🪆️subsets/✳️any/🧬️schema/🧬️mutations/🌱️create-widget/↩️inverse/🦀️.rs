//! ↩️ `create-widget` inverse — undo of a create is always a `delete-widget` by the created id
//! (per `📓️taxonomy.md`'s `create ↔ delete` pairing).

use crate::standards::v1::subsets::any::schema::mutations::create_widget::CreateWidget;
use crate::standards::v1::subsets::any::schema::mutations::delete_widget::DeleteWidget;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::{widget_id, Generation3dSnapshot};

/// ↩️ Undoing a create is deleting the same widget back out, by its own id.
pub fn inverse(payload: &CreateWidget, _base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Generation3dMutation::DeleteWidget(DeleteWidget { id: widget_id(&payload.widget).to_string() })]

    })())
}
