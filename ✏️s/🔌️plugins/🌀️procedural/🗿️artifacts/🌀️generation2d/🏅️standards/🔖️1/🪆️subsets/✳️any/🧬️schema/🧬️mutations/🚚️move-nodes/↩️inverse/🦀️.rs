//! ↩️ Inverse for `MoveNodes` — every moved widget back at its BASE position: absolute `move-widget` rows, never a negated
//! offset.

use crate::move_widget;
use semio_s_artifact_procedural_2d_model::widget_index;

use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;

pub fn inverse<Op: From<crate::move_widget::MoveWidget>>(payload: &super::MoveNodes, base: &Generation2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    if (payload.dx, payload.dy) == (0.0, 0.0) {
        return Vec::new();
    }
    payload.ids.iter().filter(|id| widget_index(&base.host_snapshot, id).is_some()).filter_map(|id| base.host_snapshot.layout.get(id).map(|layout| move_widget(id.clone(), layout.clone()))).collect()

    })())
}
