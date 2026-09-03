//! ↩️ `update-widget` inverse — captures the pre-state body from `base` and re-`update-widget`s
//! back to it (self-inverse, per `📓️taxonomy.md`'s `update` row); missing target ⇒ nothing to undo.

use crate::artifacts::generation3d::mutations::update_widget::UpdateWidget;
use crate::artifacts::generation3d::mutations::{widget_index, Generation3dMutation};
use crate::artifacts::generation3d::{widget_id, Generation3dSnapshot};

/// ↩️ Missing id in `base` ⇒ `Vec::new()`.
pub fn inverse(payload: &UpdateWidget, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
    let id = widget_id(&payload.widget);
    match widget_index(&base.fixture, id) {
        Some(index) => vec![Generation3dMutation::UpdateWidget(UpdateWidget { widget: base.fixture.widgets[index].clone() })],
        None => Vec::new(),
    }
}
