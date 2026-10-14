//! ↩️ Inverse for `ClearWidgetLayout` — restores the captured BASE layout entry, or a no-op
//! (`Vec::new()`) when the widget already had none (clearing an absent entry is itself a no-op).

use crate::move_widget;

use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;

pub fn inverse<Op: From<crate::move_widget::MoveWidget>>(payload: &super::ClearWidgetLayout, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.host_snapshot.layout.get(&payload.id) {
        Some(previous) => vec![move_widget(payload.id.clone(), previous.clone())],
        None => Vec::new(),
    }

    })())
}
