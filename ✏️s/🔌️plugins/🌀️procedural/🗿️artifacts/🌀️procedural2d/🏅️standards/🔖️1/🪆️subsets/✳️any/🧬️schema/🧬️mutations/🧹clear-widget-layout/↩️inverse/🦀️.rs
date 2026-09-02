//! ↩️ Inverse for `ClearWidgetLayout` — restores the captured BASE layout entry, or a no-op
//! (`Vec::new()`) when the widget already had none (clearing an absent entry is itself a no-op).

use crate::artifacts::procedural2d::mutations::{move_widget, Procedural2dMutation};
use crate::artifacts::procedural2d::Procedural2dSnapshot;

pub fn inverse(payload: &super::ClearWidgetLayout, base: &Procedural2dSnapshot) -> Vec<Procedural2dMutation> {
    match base.fixture.layout.get(&payload.id) {
        Some(previous) => vec![move_widget(payload.id.clone(), previous.clone())],
        None => Vec::new(),
    }
}
