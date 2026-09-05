//! ↩️ Inverse for `MoveWidget` — moves back to the captured BASE layout when the widget already had
//! one, or clears the layout entry entirely when this move is what created it (BASE had none, so
//! there is no "old position" to move back to — the true undo is removing the entry `move` added).

use crate::artifacts::generation2d::mutations::{clear_widget_layout, move_widget, Generation2dMutation};
use crate::artifacts::generation2d::Generation2dSnapshot;

pub fn inverse(payload: &super::MoveWidget, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
    match base.fixture.layout.get(&payload.id) {
        Some(previous) => vec![move_widget(payload.id.clone(), previous.clone())],
        None => vec![clear_widget_layout(payload.id.clone())],
    }
}
