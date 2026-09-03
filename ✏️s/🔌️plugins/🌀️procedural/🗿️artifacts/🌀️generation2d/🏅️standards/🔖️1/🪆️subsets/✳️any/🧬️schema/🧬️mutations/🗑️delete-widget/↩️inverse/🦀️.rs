//! ↩️ Inverse for `DeleteWidget` — recreates the removed widget at its captured BASE index, or a
//! no-op (`Vec::new()`) when the id was already absent.

use crate::artifacts::generation2d::mutations::{create_widget, Generation2dMutation};
use crate::artifacts::generation2d::{widget_id, Generation2dSnapshot};

pub fn inverse(payload: &super::DeleteWidget, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
    match base.fixture.widgets.iter().position(|widget| widget_id(widget) == payload.id) {
        Some(index) => vec![create_widget(index, base.fixture.widgets[index].clone())],
        None => Vec::new(),
    }
}
