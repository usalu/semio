//! ↩️ `move-widget` inverse — repositions back to the captured BASE-state position if one existed,
//! otherwise undoes the implied creation via `delete-widget-position`.

use crate::artifacts::generation3d::mutations::delete_widget_position::DeleteWidgetPosition;
use crate::artifacts::generation3d::mutations::move_widget::MoveWidget;
use crate::artifacts::generation3d::mutations::Generation3dMutation;
use crate::artifacts::generation3d::Generation3dSnapshot;

/// ↩️ No prior position in `base` ⇒ the upsert created the entry, so undo removes it.
pub fn inverse(payload: &MoveWidget, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
    match base.fixture.layout.get(&payload.id) {
        Some(previous) => vec![Generation3dMutation::MoveWidget(MoveWidget { id: payload.id.clone(), layout: previous.clone() })],
        None => vec![Generation3dMutation::DeleteWidgetPosition(DeleteWidgetPosition { id: payload.id.clone() })],
    }
}
