//! ↩️ Inverse for `MoveWidget` — moves back to the captured BASE layout when the widget already had
//! one, or clears the layout entry entirely when this move is what created it (BASE had none, so
//! there is no "old position" to move back to — the true undo is removing the entry `move` added).

use crate::clear_widget_layout;
use crate::move_widget;

use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;

pub fn inverse<Op: From<crate::clear_widget_layout::ClearWidgetLayout> + From<MoveWidget>>(payload: &super::MoveWidget, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.host_snapshot.layout.get(&payload.id) {
        Some(previous) => vec![move_widget(payload.id.clone(), previous.clone())],
        None => vec![clear_widget_layout(payload.id.clone())],
    }

    })())
}
