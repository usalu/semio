//! ↩️ `create-widget` inverse — undo of a create is always a `delete-widget` by the created id
//! (per `📓️taxonomy.md`'s `create ↔ delete` pairing).

use crate::artifacts::generation3d::mutations::create_widget::CreateWidget;
use crate::artifacts::generation3d::mutations::delete_widget::DeleteWidget;
use crate::artifacts::generation3d::mutations::Generation3dMutation;
use crate::artifacts::generation3d::{widget_id, Generation3dSnapshot};

/// ↩️ Undoing a create is deleting the same widget back out, by its own id.
pub fn inverse(payload: &CreateWidget, _base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
    vec![Generation3dMutation::DeleteWidget(DeleteWidget { id: widget_id(&payload.widget).to_string() })]
}
