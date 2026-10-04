//! 🔺️ `replace-points` — sparse diff construction.

use crate::{EquationDiff, EquationGeometry, EquationSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::ReplacePoints, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    if base.geometry.points == payload.points {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Points are already identical to the requested replacement.");
    }
    let geometry = EquationGeometry { points: payload.points.clone() };
    protocol::MutationOutcome::new(crate::equation_state_diff(base.graph.clone(), geometry))
}
//#endregion 🔖️Diff
