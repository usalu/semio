//! ↩️ Inverse for `MoveNodes` — every moved widget back at its BASE position: absolute `move-widget` rows, never a negated
//! offset.

use crate::standards::v1::subsets::any::schema::mutations::{move_widget, widget_index, Generation2dMutation};
use crate::Generation2dSnapshot;

pub fn inverse(payload: &super::MoveNodes, base: &Generation2dSnapshot) -> Result<Vec<Generation2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if (payload.dx, payload.dy) == (0.0, 0.0) {
        return Vec::new();
    }
    payload.ids.iter().filter(|id| widget_index(&base.host_snapshot, id).is_some()).filter_map(|id| base.host_snapshot.layout.get(id).map(|layout| move_widget(id.clone(), layout.clone()))).collect()

    })())
}
