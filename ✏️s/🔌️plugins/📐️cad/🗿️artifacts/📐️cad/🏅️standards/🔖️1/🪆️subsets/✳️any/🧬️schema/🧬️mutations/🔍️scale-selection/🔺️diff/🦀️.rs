//! 🔺️ Sparse diff builder for `ScaleSelection` — every addressed object's BASE scale is multiplied by the payload factors
//! and the pane's composed model child is re-materialized; a non-positive or non-finite factor is Fatal.
use super::ScaleSelection;
use crate::diff::CadDiff;
use crate::mutations::cad_selection_diff;
use crate::mutations::scale_objects::inverse::CAD_IDENTITY_SCALE;
use crate::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ScaleSelection, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    if payload.factors.iter().any(|factor| !(factor.is_finite() && *factor > 0.0)) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "scale factors must be finite and greater than 0", payload.targets.clone());
    }
    let factors = payload.factors;
    cad_selection_diff(payload.pane, &payload.targets, factors == CAD_IDENTITY_SCALE, base, |object| {
        let scale = object.scale.unwrap_or(CAD_IDENTITY_SCALE);
        let next = [scale[0] * factors[0], scale[1] * factors[1], scale[2] * factors[2]];
        object.scale = (next != CAD_IDENTITY_SCALE).then_some(next);
    })
}
//#endregion 🔖️Diff
