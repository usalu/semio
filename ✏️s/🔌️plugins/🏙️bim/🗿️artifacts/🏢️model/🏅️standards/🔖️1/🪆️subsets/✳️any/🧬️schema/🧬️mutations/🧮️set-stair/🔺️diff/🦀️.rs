//! 🔺️ Diff constructor for `SetStair`: a sparse stair patch naming only the provided fields that differ from the base. The patched
//! stair must hold every stair invariant again (positive width, riser and tread, sensible flight, a same-building top storey, a sound stringer, nosing, tread thickness and landing depth).

use super::super::placement::{refuse_stair, stair_issue};
use super::SetStair;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch, StairPatch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetStair, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(stair) = base.stairs.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Stair \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let patch = StairPatch {
        start: payload.start.filter(|start| *start != stair.start),
        direction: payload.direction.filter(|direction| *direction != stair.direction),
        width: payload.width.filter(|width| *width != stair.width),
        flight: payload.flight.clone().filter(|flight| *flight != stair.flight),
        top: payload.top.clone().filter(|top| *top != stair.top),
        max_riser: payload.max_riser.filter(|riser| *riser != stair.max_riser),
        min_tread: payload.min_tread.filter(|tread| *tread != stair.min_tread),
        stringer: payload.stringer.filter(|stringer| *stringer != stair.stringer),
        nosing: payload.nosing.filter(|nosing| *nosing != stair.nosing),
        tread_thickness: payload.tread_thickness.filter(|thickness| *thickness != stair.tread_thickness),
        riser: payload.riser.filter(|riser| *riser != stair.riser),
        landing_depth: payload.landing_depth.filter(|depth| *depth != stair.landing_depth),
        name: payload.name.clone().filter(|name| *name != stair.name),
        ..Default::default()
    };
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Stair \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    if let Some(issue) = stair_issue(base, &patch.write(stair)) {
        return refuse_stair(issue, &[]);
    }
    MutationOutcome::new(ModelDiff::stairs(payload.id.clone(), Entry::Patched(patch)))
}
