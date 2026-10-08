//! 🔺️ `replace-points` — sparse diff construction: the positional edits turning the cloud into the replacement.
use crate::{EquationDiff, EquationSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::ReplacePoints, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    if base.geometry.points == payload.points {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Points are already identical to the requested replacement.");
    }
    let diff = EquationDiff { points: Some(crate::diff::points_replacing(&base.geometry.points, &payload.points)), ..Default::default() };
    protocol::MutationOutcome::new(crate::equation_state_diff(diff, base))
}
//#endregion 🔖️Diff
