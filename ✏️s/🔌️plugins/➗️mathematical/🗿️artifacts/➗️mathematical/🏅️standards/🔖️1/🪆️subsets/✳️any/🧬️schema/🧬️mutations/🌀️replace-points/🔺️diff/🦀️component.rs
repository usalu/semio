//! 🔺️ `replace-points` — sparse diff construction.

use super::mutation::ReplacePoints;
use crate::artifacts::mathematical::{mathematical_children_from_state, mathematical_geometry, mathematical_graph, MathematicalDiff, MathematicalGeometry, MathematicalSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReplacePoints, base: &MathematicalSnapshot) -> protocol::MutationOutcome<MathematicalDiff> {
    if mathematical_geometry(base).points == payload.points {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Points are already identical to the requested replacement.");
    }
    let geometry = MathematicalGeometry { points: payload.points.clone() };
    let (notation, results, computed) = mathematical_children_from_state(&mathematical_graph(base), &geometry);
    protocol::MutationOutcome::new(MathematicalDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() })
}
//#endregion 🔖️Diff
