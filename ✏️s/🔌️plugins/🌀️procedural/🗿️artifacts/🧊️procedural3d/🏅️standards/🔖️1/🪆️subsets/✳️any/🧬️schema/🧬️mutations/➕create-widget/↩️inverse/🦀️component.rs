//! ↩️ `create-widget` inverse — undo of a create is always a `delete-widget` by the created id
//! (per `📓️taxonomy.md`'s `create ↔ delete` pairing).

use crate::artifacts::procedural3d::mutations::create_widget::mutation::CreateWidget;
use crate::artifacts::procedural3d::mutations::remove_widget::mutation::DeleteWidget;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;
use crate::artifacts::procedural3d::{widget_id, Procedural3dSnapshot};

/// ↩️ Undoing a create is deleting the same widget back out, by its own id.
pub fn inverse(payload: &CreateWidget, _base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
    vec![Procedural3dMutation::DeleteWidget(DeleteWidget { id: widget_id(&payload.widget).to_string() })]
}
