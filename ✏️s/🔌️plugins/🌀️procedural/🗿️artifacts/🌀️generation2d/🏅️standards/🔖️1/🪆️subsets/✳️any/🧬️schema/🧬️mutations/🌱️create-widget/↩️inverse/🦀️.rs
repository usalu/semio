//! ↩️ Inverse for `CreateWidget` — the `delete-widget` of the id it created (the payload itself
//! carries the id, so no BASE lookup is needed to know what to undo).

use crate::delete_widget;

use semio_s_artifact_procedural_2d_model::widget_id;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;

pub fn inverse<Op: From<crate::delete_widget::DeleteWidget>>(payload: &super::CreateWidget, _base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![delete_widget(widget_id(&payload.widget).to_string())]

    })())
}
