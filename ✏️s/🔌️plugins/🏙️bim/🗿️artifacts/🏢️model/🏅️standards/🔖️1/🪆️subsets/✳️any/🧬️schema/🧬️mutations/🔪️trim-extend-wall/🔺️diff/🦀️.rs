//! 🔺️ Diff constructor for `TrimExtendWall`: a wall patch with the concrete new axis (the old axis with one end moved onto the axis of the
//! target wall, see `modify::cut::trim_extend`) and, when the start moves, one offset patch per hosted opening that keeps it where it
//! stands in the world. Refused: an unknown wall or target, a wall that is its own target, axes that never meet, an end that would pass
//! the other end, a result that is no axis, and a hosted opening that no longer fits the new length; an end already on the target is
//! `mutation.no-op`.

use super::super::modify::cut::{trim_extend, TrimFlaw, Trimmed};
use super::super::modify::WallEnd;
use super::super::placement::width_of;
use super::super::wall_geometry::{flaw, snap};
use super::TrimExtendWall;
use crate::standards::v1::subsets::any::schema::authored::plan::axis_length;
use crate::{Entry, KeyedDelta, ModelDiff, ModelSnapshot, OpeningPatch, WallPatch};
use protocol::{MutationOutcome, OutcomeCode};
use std::collections::BTreeMap;

const TOLERANCE: f64 = 1e-9;

/// ✂️ Why the call is refused, or the trimmed axis and the new offset of every hosted opening that moves with the start of the wall.
pub fn plan(payload: &TrimExtendWall, base: &ModelSnapshot) -> Result<(Trimmed, BTreeMap<String, f64>), MutationOutcome<ModelDiff>> {
    let Some(wall) = base.walls.get(&payload.id) else {
        return Err(MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Wall \"{}\" does not exist.", payload.id), [payload.id.clone()]));
    };
    if payload.target == payload.id {
        return Err(MutationOutcome::refuse(OutcomeCode::Invariant, "A wall cannot be trimmed to itself.", ["target"]));
    }
    let Some(target) = base.walls.get(&payload.target) else {
        return Err(MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Wall \"{}\" does not exist.", payload.target), ["target"]));
    };
    let trimmed = trim_extend(&wall.axis, payload.end, &target.axis).map_err(|why| match why {
        TrimFlaw::Unchanged => MutationOutcome::refuse(OutcomeCode::NoOp, why.message(), [payload.id.clone()]),
        TrimFlaw::Reversed => MutationOutcome::refuse(OutcomeCode::Invariant, why.message(), ["end"]),
        TrimFlaw::Parallel | TrimFlaw::Degenerate => MutationOutcome::refuse(OutcomeCode::Invariant, why.message(), ["target"]),
    })?;
    if let Some(flaw) = flaw(&trimmed.axis) {
        return Err(flaw.under(&["end"]).refuse());
    }
    let length = axis_length(&trimmed.axis);
    let mut offsets = BTreeMap::new();
    for (id, opening) in base.openings.iter().filter(|(_, opening)| opening.host == payload.id) {
        let offset = if payload.end == WallEnd::Start { snap(opening.offset - trimmed.shift) } else { opening.offset };
        let half = width_of(base, opening).unwrap_or(0.0) / 2.0;
        if !(offset >= half - TOLERANCE && offset <= length - half + TOLERANCE) {
            return Err(MutationOutcome::refuse(OutcomeCode::Invariant, format!("Opening \"{id}\" would no longer fit the wall."), [id.clone()]));
        }
        if offset != opening.offset {
            offsets.insert(id.clone(), offset);
        }
    }
    Ok((trimmed, offsets))
}

pub fn diff(payload: &TrimExtendWall, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let (trimmed, offsets) = match plan(payload, base) {
        Ok(plan) => plan,
        Err(refusal) => return refusal,
    };
    let openings = (!offsets.is_empty()).then(|| KeyedDelta(offsets.into_iter().map(|(id, offset)| (id, Entry::Patched(OpeningPatch { offset: Some(offset), ..Default::default() }))).collect()));
    let moved = openings.as_ref().map_or(0, |delta| delta.0.len());
    let outcome = MutationOutcome::new(ModelDiff { openings, ..ModelDiff::walls(payload.id.clone(), Entry::Patched(WallPatch { axis: Some(trimmed.axis), ..Default::default() })) });
    if moved == 0 {
        outcome
    } else {
        outcome.info(OutcomeCode::Cascade, format!("{moved} opening(s) kept their place in the world."))
    }
}
