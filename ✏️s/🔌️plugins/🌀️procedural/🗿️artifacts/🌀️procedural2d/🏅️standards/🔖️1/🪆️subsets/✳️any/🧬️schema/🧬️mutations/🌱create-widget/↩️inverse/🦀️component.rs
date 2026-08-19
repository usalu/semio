//! ↩️ Inverse for `CreateWidget` — the `delete-widget` of the id it created (the payload itself
//! carries the id, so no BASE lookup is needed to know what to undo).

use crate::artifacts::procedural2d::mutations::{delete_widget, Procedural2dMutation};
use crate::artifacts::procedural2d::{widget_id, Procedural2dSnapshot};

pub async fn inverse(payload: &super::mutation::CreateWidget, _base: &Procedural2dSnapshot) -> Vec<Procedural2dMutation> {
    vec![delete_widget(widget_id(&payload.widget).to_string())]
}
