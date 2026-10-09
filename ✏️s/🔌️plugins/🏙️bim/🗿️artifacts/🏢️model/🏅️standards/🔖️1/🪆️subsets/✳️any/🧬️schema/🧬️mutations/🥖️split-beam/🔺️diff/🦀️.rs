//! 🔺️ Diff constructor for `SplitBeam`: the original beam keeps the first part of its axis, a created beam (same storey, type, phase and name) takes the second. A line splits into
//! two lines, an arc into two arcs that keep the sweep share of their arc length; an inclined beam is cut where its top is, so both parts meet at the same height. The split
//! point snaps to a nanometre. Refused: an unknown beam, a taken new id and a fraction that is not strictly inside the beam or leaves a part without length.

use super::super::elements;
use super::super::wall_geometry::{snap, split, Split};
use super::SplitBeam;
use crate::{Assigned, Beam, BeamPatch, Entry, KeyedDelta, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};
use std::collections::BTreeMap;

/// ✂️ The two parts of the axis of the beam at fraction `t` of its arc length, none when `t` is not strictly inside or a part would have no length.
pub fn middle(beam: &Beam, t: f64) -> Option<Split> {
    split(&beam.axis, t)
}

/// 📐️ The top offset where an inclined beam is cut, none for a level beam.
pub fn cut_offset(beam: &Beam, t: f64) -> Option<f64> {
    beam.end_top_offset.map(|end| snap(beam.top_offset + t * (end - beam.top_offset)))
}

pub fn diff(payload: &SplitBeam, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(beam) = base.beams.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Beam \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(noun) = elements::taken(base, &payload.new_id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.new_id), [payload.new_id.clone()]);
    }
    let Some(parts) = middle(beam, payload.t) else {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A beam splits strictly between its ends into two parts that both have length.", ["t"]);
    };
    let cut = cut_offset(beam, payload.t);
    let beams = KeyedDelta(BTreeMap::from([
        (payload.id.clone(), Entry::Patched(BeamPatch { axis: Some(parts.first), end_top_offset: cut.map(|offset| Assigned::new(Some(offset))), ..Default::default() })),
        (payload.new_id.clone(), Entry::Created(Beam { axis: parts.second, top_offset: cut.unwrap_or(beam.top_offset), ..beam.clone() })),
    ]));
    MutationOutcome::new(ModelDiff { beams: Some(beams), ..ModelDiff::default() })
}
