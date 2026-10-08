//! 🔺️ `move-points` — every addressed point moves by the payload offset from its BASE position; an index the cloud lacks
//! is skipped (`mutation.partial`).

use crate::standards::v1::subsets::geometry::schema::mutations::set_point_positions::{equation_point_targets, equation_point_targets_invariant};
use crate::diff::{canonical_point_edits, EquationPointEdit, EquationPointsDelta};
use crate::{EquationDiff, EquationPoint, EquationSnapshot};

//#region 🔖️Diff
/// 🏗️ A malformed index list or a non-finite offset is `mutation.invariant`; none left to move is
/// `mutation.target-missing`; a zero offset is `mutation.no-op`; a move off the finite canvas is `mutation.target-mismatch`.
pub fn diff(payload: &super::MovePoints, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let targets = equation_point_targets(&payload.indices);
    if let Err(reason) = equation_point_targets_invariant(&payload.indices) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, targets);
    }
    if !payload.dx.is_finite() || !payload.dy.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a point offset must be finite", targets);
    }
    let points = &base.geometry.points;
    let missing = equation_point_targets(&payload.indices.iter().copied().filter(|index| *index >= points.len()).collect::<Vec<_>>());
    if missing.len() == payload.indices.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("None of the {} point(s) exists.", payload.indices.len()), missing);
    }
    let partial = (!missing.is_empty()).then(|| protocol::MutationMessage::warning("mutation.partial", format!("{} of {} point(s) skipped (no such index): {}", missing.len(), payload.indices.len(), missing.join(", "))).at(missing));
    if (payload.dx, payload.dy) == (0.0, 0.0) {
        return protocol::MutationOutcome::empty().absorb_messages(partial.into_iter().chain([protocol::MutationMessage::warning("mutation.no-op", "The drag offset is zero.").at(targets)]));
    }
    let moved: Vec<(usize, EquationPoint)> = payload.indices.iter().filter_map(|index| points.get(*index).map(|point| (*index, EquationPoint { x: point.x + payload.dx, y: point.y + payload.dy }))).collect();
    if moved.iter().any(|(_, point)| !point.x.is_finite() || !point.y.is_finite()) {
        return protocol::MutationOutcome::error("mutation.target-mismatch", "The moved position leaves the finite canvas.", targets);
    }
    let edits = moved.into_iter().map(|(index, point)| EquationPointEdit::Set { at: index as u32, point }).collect();
    let diff = EquationDiff { points: Some(EquationPointsDelta { edits: canonical_point_edits(edits) }), ..Default::default() };
    protocol::MutationOutcome::new(crate::equation_state_diff(diff, base)).absorb_messages(partial)
}
//#endregion 🔖️Diff
