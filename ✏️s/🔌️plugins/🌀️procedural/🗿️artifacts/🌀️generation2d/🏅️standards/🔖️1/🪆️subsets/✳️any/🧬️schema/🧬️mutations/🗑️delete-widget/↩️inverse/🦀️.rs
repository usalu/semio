//! ↩️ Inverse for `DeleteWidget` — recreates the removed widget at its captured BASE index, or a
//! no-op (`Vec::new()`) when the id was already absent.

use crate::create_widget;

use semio_s_artifact_procedural_2d_model::widget_id;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;

pub fn inverse<Op: From<crate::create_widget::CreateWidget>>(payload: &super::DeleteWidget, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.host_snapshot.widgets.iter().position(|widget| widget_id(widget) == payload.id) {
        Some(index) => vec![create_widget(index, base.host_snapshot.widgets[index].clone())],
        None => Vec::new(),
    }

    })())
}
