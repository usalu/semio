//! ↩️ Inverse for `CreateWidget` — the `delete-widget` of the id it created (the payload itself
//! carries the id, so no BASE lookup is needed to know what to undo).

use crate::artifacts::generation2d::mutations::{delete_widget, Generation2dMutation};
use crate::artifacts::generation2d::{widget_id, Generation2dSnapshot};

pub fn inverse(payload: &super::CreateWidget, _base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
    vec![delete_widget(widget_id(&payload.widget).to_string())]
}
