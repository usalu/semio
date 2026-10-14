//! ↩️ `update-widget` inverse — captures the pre-state body from `base` and re-`update-widget`s
//! back to it (self-inverse, per `📓️taxonomy.md`'s `update` row); missing target ⇒ nothing to undo.

use crate::update_widget::UpdateWidget;
use semio_s_artifact_procedural_3d_model::widget_index;

use semio_s_artifact_procedural_3d_model::widget_id;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

/// ↩️ Missing id in `base` ⇒ `Vec::new()`.
pub fn inverse<Op: From<UpdateWidget>>(payload: &UpdateWidget, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let id = widget_id(&payload.widget);
    match widget_index(&base.host_snapshot, id) {
        Some(index) => vec![Op::from(UpdateWidget { widget: base.host_snapshot.widgets[index].clone() })],
        None => Vec::new(),
    }

    })())
}
