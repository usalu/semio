//! 🔺️ Diff constructor for `SetSpace`: a sparse space patch of exactly the provided fields. A new number must stay unique within the
//! storey, a new boundary must be drawable (finite seed, or an outline of three finite vertices enclosing area); providing only
//! equal values is a no-op. Outline, area and volume stay inferred.

use super::SetSpace;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch, SpaceBoundary, Vertex};
use protocol::{MutationOutcome, OutcomeCode};

fn shoelace(outline: &[Vertex]) -> f64 {
    outline.iter().zip(outline.iter().cycle().skip(1)).map(|(from, to)| from.point.x * to.point.y - to.point.x * from.point.y).sum::<f64>() / 2.0
}

fn drawable(boundary: &SpaceBoundary) -> bool {
    match boundary {
        SpaceBoundary::Bounded { seed } => seed.x.is_finite() && seed.y.is_finite(),
        SpaceBoundary::Explicit { outline } => {
            outline.len() >= 3
                && outline.iter().all(|vertex| vertex.point.x.is_finite() && vertex.point.y.is_finite() && vertex.bulge.is_finite())
                && (outline.iter().any(|vertex| vertex.bulge != 0.0) || shoelace(outline).abs() > f64::EPSILON)
        }
    }
}

pub fn diff(payload: &SetSpace, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(space) = base.spaces.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Space \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(number) = &payload.number {
        if base.spaces.iter().any(|(other, row)| *other != payload.id && row.storey == space.storey && row.number == *number) {
            return MutationOutcome::refuse(OutcomeCode::Invariant, format!("Space number \"{number}\" is already used on storey \"{}\".", space.storey), ["number"]);
        }
    }
    if payload.boundary.as_ref().is_some_and(|boundary| !drawable(boundary)) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A space boundary needs a finite seed or an outline of at least three finite vertices enclosing area.", ["boundary"]);
    }
    let patch = payload.patch().minimal(space);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Space \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::spaces(payload.id.clone(), Entry::Patched(patch)))
}
