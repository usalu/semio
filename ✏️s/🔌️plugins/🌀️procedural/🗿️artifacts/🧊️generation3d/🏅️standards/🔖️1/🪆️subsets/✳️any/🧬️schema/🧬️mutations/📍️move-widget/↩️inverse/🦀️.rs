//! ↩️ `move-widget` inverse — repositions back to the captured BASE-state position if one existed,
//! otherwise undoes the implied creation via `delete-widget-position`.

use crate::standards::v1::subsets::any::schema::mutations::delete_widget_position::DeleteWidgetPosition;
use crate::standards::v1::subsets::any::schema::mutations::move_widget::MoveWidget;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;

/// ↩️ No prior position in `base` ⇒ the upsert created the entry, so undo removes it.
pub fn inverse(payload: &MoveWidget, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
    match base.fixture.layout.get(&payload.id) {
        Some(previous) => vec![Generation3dMutation::MoveWidget(MoveWidget { id: payload.id.clone(), layout: previous.clone() })],
        None => vec![Generation3dMutation::DeleteWidgetPosition(DeleteWidgetPosition { id: payload.id.clone() })],
    }
}
