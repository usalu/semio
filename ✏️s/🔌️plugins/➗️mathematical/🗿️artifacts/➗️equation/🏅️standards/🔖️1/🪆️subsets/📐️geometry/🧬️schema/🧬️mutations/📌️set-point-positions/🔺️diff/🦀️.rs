//! 🔺️ `set-point-positions` — every addressed point lands at its payload position; an index the cloud lacks is skipped
//! (`mutation.partial`).

use super::{equation_point_targets, equation_point_targets_invariant, SetPointPositions};
use crate::diff::{canonical_point_edits, EquationPointEdit, EquationPointsDelta};
use crate::{EquationDiff, EquationPoint, EquationSnapshot};

//#region 🔖️Diff
/// 🏗️ A malformed position list is `mutation.invariant`; none left to place is `mutation.target-missing`; every point
/// already in place is `mutation.no-op`.
pub fn diff(payload: &SetPointPositions, base: &EquationSnapshot) -> protocol::MutationOutcome<EquationDiff> {
    let indices = payload.indices();
    let targets = equation_point_targets(&indices);
    if let Err(reason) = equation_point_targets_invariant(&indices) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, targets);
    }
    if payload.positions.iter().any(|position| !position.x.is_finite() || !position.y.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A point position must be finite.", targets);
    }
    let points = &base.geometry.points;
    let missing = equation_point_targets(&indices.iter().copied().filter(|index| *index >= points.len()).collect::<Vec<_>>());
    if missing.len() == indices.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("None of the {} point(s) exists.", indices.len()), missing);
    }
    let partial = (!missing.is_empty()).then(|| protocol::MutationMessage::warning("mutation.partial", format!("{} of {} point(s) skipped (no such index): {}", missing.len(), indices.len(), missing.join(", "))).at(missing));
    let edits: Vec<EquationPointEdit> = payload
        .positions
        .iter()
        .filter(|position| points.get(position.index).is_some_and(|point| (point.x, point.y) != (position.x, position.y)))
        .map(|position| EquationPointEdit::Set { at: position.index as u32, point: EquationPoint { x: position.x, y: position.y } })
        .collect();
    if edits.is_empty() {
        return protocol::MutationOutcome::empty().absorb_messages(partial.into_iter().chain([protocol::MutationMessage::warning("mutation.no-op", "Every point already sits at its position.").at(targets)]));
    }
    let diff = EquationDiff { points: Some(EquationPointsDelta { edits: canonical_point_edits(edits) }), ..Default::default() };
    protocol::MutationOutcome::new(crate::equation_state_diff(diff, base)).absorb_messages(partial)
}
//#endregion 🔖️Diff
