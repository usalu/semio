//! 🔺️ Diff constructor for `CreateSpace`: one created space entry. The storey must exist, the number is unique within the storey, a
//! bounded space needs a finite seed and an explicit outline at least three finite vertices that enclose area. Outline, area and
//! volume of a bounded space are inferred from the walls around the seed.

use super::super::elements;
use super::CreateSpace;
use crate::{Entry, ModelDiff, ModelSnapshot, SpaceBoundary, Vertex};
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

pub fn diff(payload: &CreateSpace, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let space = &payload.space;
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !base.storeys.contains_key(&space.storey) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{}\" does not exist.", space.storey), ["space", "storey"]);
    }
    if base.spaces.values().any(|row| row.storey == space.storey && row.number == space.number) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, format!("Space number \"{}\" is already used on storey \"{}\".", space.number, space.storey), ["space", "number"]);
    }
    if !drawable(&space.boundary) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A space boundary needs a finite seed or an outline of at least three finite vertices enclosing area.", ["space", "boundary"]);
    }
    MutationOutcome::new(ModelDiff::spaces(payload.id.clone(), Entry::Created(space.clone())))
}
