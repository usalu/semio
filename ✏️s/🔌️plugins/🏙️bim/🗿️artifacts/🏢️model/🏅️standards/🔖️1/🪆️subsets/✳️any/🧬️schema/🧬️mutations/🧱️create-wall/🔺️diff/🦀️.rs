//! 🔺️ Diff constructor for `CreateWall`: one created wall entry. The storey and the wall type must exist, a storey constraint
//! must name a storey of the same building, and the axis must have length. No height is stored: it is inferred.

use super::super::elements;
use super::CreateWall;
use crate::{Axis, Entry, ModelDiff, ModelSnapshot, TopConstraint};
use protocol::{MutationOutcome, OutcomeCode};

fn degenerate(axis: &Axis) -> bool {
    let (Axis::Line { start, end } | Axis::Arc { start, end, .. }) = axis;
    start == end
}

pub fn diff(payload: &CreateWall, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let wall = &payload.wall;
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    let Some(storey) = base.storeys.get(&wall.storey) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{}\" does not exist.", wall.storey), ["wall", "storey"]);
    };
    if !base.wall_types.contains_key(&wall.wall_type) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Wall type \"{}\" does not exist.", wall.wall_type), ["wall", "wall_type"]);
    }
    if let TopConstraint::Storey { storey: target, .. } = &wall.top {
        match base.storeys.get(target) {
            None => return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{target}\" does not exist."), ["wall", "top", "storey"]),
            Some(row) if row.building != storey.building => return MutationOutcome::refuse(OutcomeCode::Invariant, format!("Storey \"{target}\" belongs to another building."), ["wall", "top", "storey"]),
            Some(_) => {}
        }
    }
    if degenerate(&wall.axis) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A wall axis must have length.", ["wall", "axis"]);
    }
    MutationOutcome::new(ModelDiff::walls(payload.id.clone(), Entry::Created(wall.clone())))
}
