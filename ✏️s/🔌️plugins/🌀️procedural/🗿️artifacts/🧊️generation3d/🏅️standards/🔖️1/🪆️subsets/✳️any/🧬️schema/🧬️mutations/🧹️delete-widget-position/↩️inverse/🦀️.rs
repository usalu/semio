//! ↩️ `delete-widget-position` inverse — reconstructs a `move-widget` from BASE state; a position
//! already absent from `base` has nothing to undo.

use crate::delete_widget_position::DeleteWidgetPosition;
use crate::move_widget::MoveWidget;
use crate::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

/// ↩️ Missing id in `base` ⇒ `Vec::new()`.
pub fn inverse(payload: &DeleteWidgetPosition, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.host_snapshot.layout.get(&payload.id) {
        Some(previous) => vec![Generation3dMutation::MoveWidget(MoveWidget { id: payload.id.clone(), layout: previous.clone() })],
        None => Vec::new(),
    }

    })())
}
