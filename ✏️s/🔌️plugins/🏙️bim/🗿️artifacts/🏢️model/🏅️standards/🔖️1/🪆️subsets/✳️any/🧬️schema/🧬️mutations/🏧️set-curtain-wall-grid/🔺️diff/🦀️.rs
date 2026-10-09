//! 🔺️ Diff constructor for `SetCurtainWallGrid`: a sparse curtain wall patch of the provided grid overrides that differ. A provided rule is a uniform spacing (positive) or a
//! list of grid lines at positive distances in strictly ascending order; an assigned none clears the override so the wall follows its type again. The grid lines are ONE list owned
//! by the curtain wall: a provided list replaces the list as a whole, its lines have no identity of their own. Cells, panels and overrides follow by inference.

use super::super::wall_geometry::grid_flaw;
use super::SetCurtainWallGrid;
use crate::{CurtainWallPatch, Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetCurtainWallGrid, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(wall) = base.curtain_walls.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Curtain wall \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    for (field, assigned) in [("u_grid", &payload.u_grid), ("v_grid", &payload.v_grid)] {
        if let Some(flaw) = assigned.as_ref().and_then(|assigned| assigned.value.as_ref()).and_then(grid_flaw) {
            return flaw.under(&[field]).refuse();
        }
    }
    let change = payload.patch().minimal(wall);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Curtain wall \"{}\" already has this grid.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::curtain_walls(payload.id.clone(), Entry::Patched(change)))
}
