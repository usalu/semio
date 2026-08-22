//! ↩️ `delete-widget` inverse — reconstructs a `create-widget` from BASE state (never from the
//! diff); a widget already absent from `base` has nothing to undo.

use crate::artifacts::procedural3d::mutations::create_widget::mutation::CreateWidget;
use crate::artifacts::procedural3d::mutations::delete_widget::mutation::DeleteWidget;
use crate::artifacts::procedural3d::mutations::{widget_index, Procedural3dMutation};
use crate::artifacts::procedural3d::Procedural3dSnapshot;

/// ↩️ Missing id in `base` ⇒ `Vec::new()` (nothing to undo).
pub fn inverse(payload: &DeleteWidget, base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
    match widget_index(&base.fixture, &payload.id) {
        Some(index) => vec![Procedural3dMutation::CreateWidget(CreateWidget { index, widget: base.fixture.widgets[index].clone() })],
        None => Vec::new(),
    }
}
