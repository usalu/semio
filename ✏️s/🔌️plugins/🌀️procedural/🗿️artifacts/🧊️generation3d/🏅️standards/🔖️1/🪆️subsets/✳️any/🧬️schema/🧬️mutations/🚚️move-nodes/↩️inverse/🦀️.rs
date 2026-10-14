//! ↩️ `move-nodes` inverse — every moved widget back at its BASE position: absolute `move-widget` rows, never a negated
//! offset.

use crate::move_nodes::MoveNodes;
use crate::move_widget::MoveWidget;
use crate::widget_index;
use crate::Generation3dMutation;

use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;

/// ↩️ One `move-widget` per addressed widget that has a stored base position.
pub fn inverse(payload: &MoveNodes, base: &Generation3dSnapshot) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if (payload.dx, payload.dy) == (0.0, 0.0) {
        return Vec::new();
    }
    payload.ids.iter().filter(|id| widget_index(&base.host_snapshot, id).is_some()).filter_map(|id| base.host_snapshot.layout.get(id).map(|layout| Generation3dMutation::MoveWidget(MoveWidget { id: id.clone(), layout: layout.clone() }))).collect()

    })())
}
