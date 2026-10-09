//! 🔺️ Diff constructor for `SetCurtainWallType`: a sparse curtain wall type patch of exactly the provided fields that differ. Provided grid rules, mullion
//! sections, default panel and materials follow the create rules; a patch that changes nothing is a no-op. Every curtain wall of the type follows by inference.

use super::super::wall_geometry::{grid_flaw, material_flaw, panel_flaw, profile_flaw, Flaw};
use super::SetCurtainWallType;
use crate::{Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

fn flaw_of(payload: &SetCurtainWallType, base: &ModelSnapshot) -> Option<Flaw> {
    payload.u_grid.as_ref().and_then(grid_flaw).map(|flaw| flaw.under(&["u_grid"]))
        .or_else(|| payload.v_grid.as_ref().and_then(grid_flaw).map(|flaw| flaw.under(&["v_grid"])))
        .or_else(|| payload.interior_mullion.as_ref().and_then(profile_flaw).map(|flaw| flaw.under(&["interior_mullion"])))
        .or_else(|| payload.border_mullion.as_ref().and_then(profile_flaw).map(|flaw| flaw.under(&["border_mullion"])))
        .or_else(|| payload.panel.as_ref().and_then(|panel| panel_flaw(base, panel)).map(|flaw| flaw.under(&["panel"])))
        .or_else(|| payload.panel_material.as_deref().and_then(|id| material_flaw(base, "panel_material", id)))
        .or_else(|| payload.mullion_material.as_deref().and_then(|id| material_flaw(base, "mullion_material", id)))
}

pub fn diff(payload: &SetCurtainWallType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(current) = base.curtain_wall_types.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Curtain wall type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(flaw) = flaw_of(payload, base) {
        return flaw.refuse();
    }
    let change = payload.patch().minimal(current);
    if change.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Curtain wall type \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::curtain_wall_types(payload.id.clone(), Entry::Patched(change)))
}
