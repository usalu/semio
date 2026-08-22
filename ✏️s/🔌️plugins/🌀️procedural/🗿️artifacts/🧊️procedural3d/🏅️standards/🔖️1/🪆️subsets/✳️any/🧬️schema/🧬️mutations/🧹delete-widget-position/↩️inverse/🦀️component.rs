//! ↩️ `delete-widget-position` inverse — reconstructs a `move-widget` from BASE state; a position
//! already absent from `base` has nothing to undo.

use crate::artifacts::procedural3d::mutations::delete_widget_position::mutation::DeleteWidgetPosition;
use crate::artifacts::procedural3d::mutations::move_widget::mutation::MoveWidget;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;

/// ↩️ Missing id in `base` ⇒ `Vec::new()`.
pub fn inverse(payload: &DeleteWidgetPosition, base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
    match base.fixture.layout.get(&payload.id) {
        Some(previous) => vec![Procedural3dMutation::MoveWidget(MoveWidget { id: payload.id.clone(), layout: previous.clone() })],
        None => Vec::new(),
    }
}
