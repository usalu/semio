//! ↩️ `move-nodes` inverse — every moved widget back at its BASE position: absolute `move-widget` rows, never a negated
//! offset.

use crate::move_nodes::MoveNodes;
use crate::move_widget::MoveWidget;
use semio_s_artifact_procedural_3d_model::widget_index;

use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

/// ↩️ One `move-widget` per addressed widget that has a stored base position.
pub fn inverse<Op: From<crate::move_widget::MoveWidget>>(payload: &MoveNodes, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    if (payload.dx, payload.dy) == (0.0, 0.0) {
        return Vec::new();
    }
    payload.ids.iter().filter(|id| widget_index(&base.host_snapshot, id).is_some()).filter_map(|id| base.host_snapshot.layout.get(id).map(|layout| Op::from(MoveWidget { id: id.clone(), layout: layout.clone() }))).collect()

    })())
}
