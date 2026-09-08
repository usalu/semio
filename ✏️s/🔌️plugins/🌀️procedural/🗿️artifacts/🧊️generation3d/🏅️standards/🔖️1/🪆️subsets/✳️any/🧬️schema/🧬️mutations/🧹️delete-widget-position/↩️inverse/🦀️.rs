//! ↩️ `delete-widget-position` inverse — reconstructs a `move-widget` from BASE state; a position
//! already absent from `base` has nothing to undo.

use crate::mutations::delete_widget_position::DeleteWidgetPosition;
use crate::mutations::move_widget::MoveWidget;
use crate::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;

/// ↩️ Missing id in `base` ⇒ `Vec::new()`.
pub fn inverse(payload: &DeleteWidgetPosition, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
    match base.fixture.layout.get(&payload.id) {
        Some(previous) => vec![Generation3dMutation::MoveWidget(MoveWidget { id: payload.id.clone(), layout: previous.clone() })],
        None => Vec::new(),
    }
}
