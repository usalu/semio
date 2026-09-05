//! 🔺️ `replace-points` — sparse diff construction.

use crate::artifacts::equation::{equation_children_from_state, equation_geometry, equation_graph, EquationDiff, EquationGeometry, EquationSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &super::ReplacePoints, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    if equation_geometry(base).points == payload.points {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Points are already identical to the requested replacement.");
    }
    let geometry = EquationGeometry { points: payload.points.clone() };
    let (notation, results, computed) = equation_children_from_state(&equation_graph(base), &geometry);
    protocol::MutationOutcome::new(EquationDiff { notation: Some(notation), results: Some(results), computed: Some(computed), ..Default::default() })
}
//#endregion 🔖️Diff
