//! ↩️ `move-nodes` inverse — every moved widget back at its BASE position: absolute `move-widget` rows, never a negated
//! offset.

use crate::standards::v1::subsets::any::schema::mutations::move_nodes::MoveNodes;
use crate::standards::v1::subsets::any::schema::mutations::move_widget::MoveWidget;
use crate::standards::v1::subsets::any::schema::mutations::{widget_index, Generation3dMutation};
use crate::Generation3dSnapshot;

/// ↩️ One `move-widget` per addressed widget that has a stored base position.
pub fn inverse(payload: &MoveNodes, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
    if (payload.dx, payload.dy) == (0.0, 0.0) {
        return Vec::new();
    }
    payload.ids.iter().filter(|id| widget_index(&base.host_snapshot, id).is_some()).filter_map(|id| base.host_snapshot.layout.get(id).map(|layout| Generation3dMutation::MoveWidget(MoveWidget { id: id.clone(), layout: layout.clone() }))).collect()
}
