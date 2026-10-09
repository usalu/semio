//! 🔺️ Diff constructor for `AlignElements`: one sparse placement patch per element that has to move so that the chosen edge (lower, middle
//! or upper) of its authored extent on the chosen coordinate lies on the target. Arcs count with their extremes; hosted openings follow
//! their host and are never patched. Refused: an empty selection, unknown ids, elements without a placement and a target that is not
//! finite; elements already on the target are left out and a selection with none to move is `mutation.no-op`.

use super::super::elements::{self, Change, Refusal};
use super::super::modify::map::{align_gap, bounds};
use super::super::modify::AlignAxis;
use super::AlignElements;
use crate::{ModelDiff, ModelSnapshot, Point2};
use protocol::{MutationOutcome, OutcomeCode};

/// 🧲️ The placements the alignment writes: every element translated along the coordinate by the gap between its edge and the target.
pub fn changes(payload: &AlignElements, base: &ModelSnapshot) -> Result<Change, Refusal> {
    if !payload.target.is_finite() {
        return Err(Refusal::new(OutcomeCode::Invariant, "An alignment target must be a finite coordinate.", ["target"]));
    }
    elements::changes_by(base, &payload.ids, |_, placement| {
        let Some(extent) = bounds(placement) else { return placement.clone() };
        let gap = align_gap(extent, payload.axis, payload.edge, payload.target);
        placement.translated(if payload.axis == AlignAxis::X { Point2 { x: gap, y: 0.0 } } else { Point2 { x: 0.0, y: gap } })
    })
}

pub fn diff(payload: &AlignElements, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    elements::placed(changes(payload, base), "ids")
}
