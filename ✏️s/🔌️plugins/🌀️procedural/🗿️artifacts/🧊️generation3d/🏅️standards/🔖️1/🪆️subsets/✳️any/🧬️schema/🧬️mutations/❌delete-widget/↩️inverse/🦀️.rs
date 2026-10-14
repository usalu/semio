//! ↩️ `delete-widget` inverse — reconstructs a `create-widget` from BASE state (never from the
//! diff); a widget already absent from `base` has nothing to undo.

use crate::create_widget::CreateWidget;
use crate::delete_widget::DeleteWidget;
use crate::widget_index;
use crate::Generation3dMutation;

use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

/// ↩️ Missing id in `base` ⇒ `Vec::new()` (nothing to undo).
pub fn inverse(payload: &DeleteWidget, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match widget_index(&base.host_snapshot, &payload.id) {
        Some(index) => vec![Generation3dMutation::CreateWidget(CreateWidget { index, widget: base.host_snapshot.widgets[index].clone() })],
        None => Vec::new(),
    }

    })())
}
