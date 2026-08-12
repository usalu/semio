//! ↩️ `move-widget` inverse — repositions back to the captured BASE-state position if one existed,
//! otherwise undoes the implied creation via `delete-widget-position`.

use crate::artifacts::procedural3d::mutations::delete_widget_position::mutation::DeleteWidgetPosition;
use crate::artifacts::procedural3d::mutations::move_widget::mutation::MoveWidget;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;

/// ↩️ No prior position in `base` ⇒ the upsert created the entry, so undo removes it.
pub fn inverse(payload: &MoveWidget, base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
    match base.fixture.layout.get(&payload.id) {
        Some(previous) => vec![Procedural3dMutation::MoveWidget(MoveWidget { id: payload.id.clone(), layout: previous.clone() })],
        None => vec![Procedural3dMutation::DeleteWidgetPosition(DeleteWidgetPosition { id: payload.id.clone() })]}
}
