//! 🔺️ Diff constructor for `SplitSlab`: the slab keeps the piece to the left of the cut line (seen from its first point to its second),
//! a created slab (same storey, type, offset, slope, phase and name) takes the piece to the right, and every hole goes with the piece it
//! lies in. The line must cross the outline exactly twice and no hole; split arcs keep their circle and both pieces must be valid slab
//! loops. Refused: an unknown slab, a taken new id, a cut line without length, a line that does not cut the outline in two and a line
//! through a hole.

use super::super::elements;
use super::super::horizontal_rules::{holes_fault, loop_fault};
use super::super::modify::cut::{contains, crosses, cut_loop, CutFlaw};
use super::SplitSlab;
use crate::{Entry, KeyedDelta, ModelDiff, ModelSnapshot, Slab, SlabPatch, Vertex};
use protocol::{MutationOutcome, OutcomeCode};
use std::collections::BTreeMap;

/// ✂️ The pieces of the slab: the left loop with its holes and the right loop with its holes, or the refusal.
pub fn pieces(payload: &SplitSlab, base: &ModelSnapshot) -> Result<((Vec<Vertex>, Vec<Vec<Vertex>>), (Vec<Vertex>, Vec<Vec<Vertex>>)), MutationOutcome<ModelDiff>> {
    let Some(slab) = base.slabs.get(&payload.id) else {
        return Err(MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Slab \"{}\" does not exist.", payload.id), [payload.id.clone()]));
    };
    let (start, end) = (payload.line_start, payload.line_end);
    if ![start.x, start.y, end.x, end.y].iter().all(|value| value.is_finite()) {
        return Err(MutationOutcome::refuse(OutcomeCode::Invariant, "A cut line needs finite points.", ["line_start"]));
    }
    if start == end {
        return Err(MutationOutcome::refuse(OutcomeCode::Invariant, CutFlaw::Line.message(), ["line_end"]));
    }
    let cut = cut_loop(&slab.boundary, start, end).map_err(|flaw| MutationOutcome::refuse(OutcomeCode::Invariant, flaw.message(), ["line_start"]))?;
    let (mut left_holes, mut right_holes) = (Vec::new(), Vec::new());
    for hole in &slab.holes {
        if crosses(hole, start, end) {
            return Err(MutationOutcome::refuse(OutcomeCode::Invariant, "The cut line must not cross a hole.", ["holes"]));
        }
        if contains(&cut.left, hole[0].point) { left_holes.push(hole.clone()) } else { right_holes.push(hole.clone()) }
    }
    for (piece, holes) in [(&cut.left, &left_holes), (&cut.right, &right_holes)] {
        if let Some(fault) = loop_fault(piece) {
            return Err(MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["line_start"]));
        }
        if let Some(fault) = holes_fault(piece, holes) {
            return Err(MutationOutcome::refuse(OutcomeCode::Invariant, fault, ["holes"]));
        }
    }
    Ok(((cut.left, left_holes), (cut.right, right_holes)))
}

pub fn diff(payload: &SplitSlab, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.new_id).filter(|_| base.slabs.contains_key(&payload.id)) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.new_id), [payload.new_id.clone()]);
    }
    let ((boundary, holes), (right, right_holes)) = match pieces(payload, base) {
        Ok(found) => found,
        Err(refusal) => return refusal,
    };
    let slab = &base.slabs[&payload.id];
    let kept = SlabPatch { boundary: Some(boundary), holes: (holes != slab.holes).then_some(holes), ..Default::default() };
    let slabs = KeyedDelta(BTreeMap::from([(payload.id.clone(), Entry::Patched(kept)), (payload.new_id.clone(), Entry::Created(Slab { boundary: right, holes: right_holes, ..slab.clone() }))]));
    MutationOutcome::new(ModelDiff { slabs: Some(slabs), ..ModelDiff::default() })
}
