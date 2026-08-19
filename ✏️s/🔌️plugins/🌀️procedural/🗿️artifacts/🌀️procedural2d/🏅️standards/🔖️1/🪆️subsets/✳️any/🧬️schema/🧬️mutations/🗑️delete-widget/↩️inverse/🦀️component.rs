//! ↩️ Inverse for `DeleteWidget` — recreates the removed widget at its captured BASE index, or a
//! no-op (`Vec::new()`) when the id was already absent.

use crate::artifacts::procedural2d::mutations::{create_widget, Procedural2dMutation};
use crate::artifacts::procedural2d::{widget_id, Procedural2dSnapshot};

pub async fn inverse(payload: &super::mutation::DeleteWidget, base: &Procedural2dSnapshot) -> Vec<Procedural2dMutation> {
    match base.fixture.widgets.iter().position(|widget| widget_id(widget) == payload.id) {
        Some(index) => vec![create_widget(index, base.fixture.widgets[index].clone())],
        None => Vec::new(),
    }
}
