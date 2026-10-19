//! 🔺️ Diff constructor for `AlignElements`: one sparse placement patch per element that has to move so that the chosen edge (lower, middle
//! or upper) of its authored extent on the chosen coordinate lies on the target. Arcs count with their extremes; hosted openings follow
//! their host and are never patched. Refused: an empty selection, unknown ids, elements without a placement and a target that is not
//! finite; elements already on the target are left out and a selection with none to move is `mutation.no-op`.

use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::{Change, Refusal};
use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::schema::mutations::modify::map::{align_gap, bounds};
use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::schema::mutations::modify::AlignAxis;
use crate::standards::v1::subsets::any::schema::mutations::align_elements::AlignElements;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Point2;
use protocol::{MutationOutcome, OutcomeCode};

/// 🧲️ The placements the alignment writes: every element translated along the coordinate by the gap between its edge and the target.
pub fn changes(payload: &AlignElements, base: &ModelSnapshot) -> Result<Change, Refusal> {
    if !payload.target.is_finite() {
        return Err(Refusal::new(OutcomeCode::Invariant, "An alignment target must be a finite coordinate.", ["target"]));
    }
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::changes_by(base, &payload.ids, |_, placement| {
        let Some(extent) = bounds(placement) else { return placement.clone() };
        let gap = align_gap(extent, payload.axis, payload.edge, payload.target);
        placement.translated(if payload.axis == AlignAxis::X { Point2 { x: gap, y: 0.0 } } else { Point2 { x: 0.0, y: gap } })
    })
}

pub fn diff(payload: &AlignElements, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::placed(changes(payload, base), "ids")
}
