//! ↩️ `delete-widget` inverse — reconstructs a `create-widget` from BASE state (never from the
//! diff); a widget already absent from `base` has nothing to undo.

use crate::create_widget::CreateWidget;
use crate::delete_widget::DeleteWidget;
use semio_s_artifact_procedural_3d_model::widget_index;

use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

/// ↩️ Missing id in `base` ⇒ `Vec::new()` (nothing to undo).
pub fn inverse<Op: From<crate::create_widget::CreateWidget>>(payload: &DeleteWidget, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    match widget_index(&base.host_snapshot, &payload.id) {
        Some(index) => vec![Op::from(CreateWidget { index, widget: base.host_snapshot.widgets[index].clone() })],
        None => Vec::new(),
    }

    })())
}
