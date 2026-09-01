//! ↩️ `update-widget` inverse — captures the pre-state body from `base` and re-`update-widget`s
//! back to it (self-inverse, per `📓️taxonomy.md`'s `update` row); missing target ⇒ nothing to undo.

use crate::artifacts::procedural3d::mutations::update_widget::UpdateWidget;
use crate::artifacts::procedural3d::mutations::{widget_index, Procedural3dMutation};
use crate::artifacts::procedural3d::{widget_id, Procedural3dSnapshot};

/// ↩️ Missing id in `base` ⇒ `Vec::new()`.
pub fn inverse(payload: &UpdateWidget, base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
    let id = widget_id(&payload.widget);
    match widget_index(&base.fixture, id) {
        Some(index) => vec![Procedural3dMutation::UpdateWidget(UpdateWidget { widget: base.fixture.widgets[index].clone() })],
        None => Vec::new(),
    }
}
