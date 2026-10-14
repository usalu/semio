//! ↩️ `create-widget` inverse — undo of a create is always a `delete-widget` by the created id
//! (per `📓️taxonomy.md`'s `create ↔ delete` pairing).

use crate::create_widget::CreateWidget;
use crate::delete_widget::DeleteWidget;
use crate::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::widget_id;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

/// ↩️ Undoing a create is deleting the same widget back out, by its own id.
pub fn inverse(payload: &CreateWidget, _base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Generation3dMutation::DeleteWidget(DeleteWidget { id: widget_id(&payload.widget).to_string() })]

    })())
}
