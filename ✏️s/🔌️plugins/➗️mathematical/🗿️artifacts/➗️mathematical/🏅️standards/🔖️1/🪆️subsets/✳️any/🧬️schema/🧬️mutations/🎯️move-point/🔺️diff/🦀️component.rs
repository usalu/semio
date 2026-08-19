//! 🔺️ `move-point` — sparse diff construction.

use super::mutation::MovePoint;
use crate::artifacts::mathematical::{mathematical_children_from_state, mathematical_geometry, mathematical_graph, MathematicalDiff, MathematicalSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &MovePoint, base: &MathematicalSnapshot) -> protocol::MutationOutcome<MathematicalDiff> {
    let mut geometry = mathematical_geometry(base);
    let Some(existing) = geometry.points.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Point at index {} does not exist.", payload.index), [payload.index.to_string()]);
    };
    if !payload.x.is_finite() || !payload.y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Point at index {} position must be finite, got ({}, {}).", payload.index, payload.x, payload.y), [payload.index.to_string()]);
    }
    if existing.x == payload.x && existing.y == payload.y {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Point at index {} is already at ({}, {}).", payload.index, payload.x, payload.y));
    }
    if let Some(point) = geometry.points.get_mut(payload.index) {
        point.x = payload.x;
        point.y = payload.y;
    }
    let (notation, results, computed) = mathematical_children_from_state(&mathematical_graph(base), &geometry);
    protocol::MutationOutcome::new(MathematicalDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() })
}
//#endregion 🔖️Diff
