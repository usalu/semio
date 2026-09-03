//! ↩️ `delete-widget` inverse — reconstructs a `create-widget` from BASE state (never from the
//! diff); a widget already absent from `base` has nothing to undo.

use crate::artifacts::generation3d::mutations::create_widget::CreateWidget;
use crate::artifacts::generation3d::mutations::delete_widget::DeleteWidget;
use crate::artifacts::generation3d::mutations::{widget_index, Generation3dMutation};
use crate::artifacts::generation3d::Generation3dSnapshot;

/// ↩️ Missing id in `base` ⇒ `Vec::new()` (nothing to undo).
pub fn inverse(payload: &DeleteWidget, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
    match widget_index(&base.fixture, &payload.id) {
        Some(index) => vec![Generation3dMutation::CreateWidget(CreateWidget { index, widget: base.fixture.widgets[index].clone() })],
        None => Vec::new(),
    }
}
