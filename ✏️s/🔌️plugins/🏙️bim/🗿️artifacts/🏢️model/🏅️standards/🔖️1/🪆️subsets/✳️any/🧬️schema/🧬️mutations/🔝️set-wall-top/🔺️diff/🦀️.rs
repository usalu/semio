//! 🔺️ Diff constructor for `SetWallTop`: a one-field wall patch. A storey constraint must name a storey of the wall's building;
//! the resolved height is inferred, never written.

use super::SetWallTop;
use crate::{Entry, ModelDiff, ModelSnapshot, TopConstraint, WallPatch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetWallTop, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(wall) = base.walls.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Wall \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let TopConstraint::Storey { storey: target, .. } = &payload.top {
        let own = base.storeys.get(&wall.storey).map(|row| &row.building);
        match base.storeys.get(target) {
            None => return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{target}\" does not exist."), ["top", "storey"]),
            Some(row) if Some(&row.building) != own => return MutationOutcome::refuse(OutcomeCode::Invariant, format!("Storey \"{target}\" belongs to another building."), ["top", "storey"]),
            Some(_) => {}
        }
    }
    if wall.top == payload.top {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Wall \"{}\" already has this top.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::walls(payload.id.clone(), Entry::Patched(WallPatch { top: Some(payload.top.clone()), ..Default::default() })))
}
