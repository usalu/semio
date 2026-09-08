//! ↩️ Inverse for `ClearWidgetLayout` — restores the captured BASE layout entry, or a no-op
//! (`Vec::new()`) when the widget already had none (clearing an absent entry is itself a no-op).

use crate::standards::v1::subsets::any::schema::mutations::{move_widget, Generation2dMutation};
use crate::Generation2dSnapshot;

pub fn inverse(payload: &super::ClearWidgetLayout, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
    match base.fixture.layout.get(&payload.id) {
        Some(previous) => vec![move_widget(payload.id.clone(), previous.clone())],
        None => Vec::new(),
    }
}
