//! ↩️ `move-widget` inverse — repositions back to the captured BASE-state position if one existed,
//! otherwise undoes the implied creation via `delete-widget-position`.

use crate::delete_widget_position::DeleteWidgetPosition;
use crate::move_widget::MoveWidget;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

/// ↩️ No prior position in `base` ⇒ the upsert created the entry, so undo removes it.
pub fn inverse<Op: From<MoveWidget> + From<crate::delete_widget_position::DeleteWidgetPosition>>(payload: &MoveWidget, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.host_snapshot.layout.get(&payload.id) {
        Some(previous) => vec![Op::from(MoveWidget { id: payload.id.clone(), layout: previous.clone() })],
        None => vec![Op::from(DeleteWidgetPosition { id: payload.id.clone() })],
    }

    })())
}
