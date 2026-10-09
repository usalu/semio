//! 🔺️ Diff constructor for `CreateCurtainPanelOverride`: one created override entry. The curtain wall must exist, a solid panel names an existing material, a door or
//! window panel an existing type, and the cell (curtain wall, u, v) is overridden at most once. Whether the cell lies inside the inferred grid is a diagnostic, never a
//! refusal: the grid follows the type and the wall and may change under the override.

use super::super::elements;
use super::super::wall_geometry::panel_flaw;
use super::CreateCurtainPanelOverride;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateCurtainPanelOverride, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let record = &payload.curtain_panel_override;
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !base.curtain_walls.contains_key(&record.curtain) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Curtain wall \"{}\" does not exist.", record.curtain), ["curtain_panel_override", "curtain"]);
    }
    if let Some(flaw) = panel_flaw(base, &record.panel) {
        return flaw.under(&["curtain_panel_override", "panel"]).refuse();
    }
    if let Some((held, _)) = base.curtain_panel_overrides.iter().find(|(_, row)| row.curtain == record.curtain && row.u == record.u && row.v == record.v) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("Cell ({}, {}) of curtain wall \"{}\" is already overridden by \"{held}\".", record.u, record.v, record.curtain), [held.clone()]);
    }
    MutationOutcome::new(ModelDiff::curtain_panel_overrides(payload.id.clone(), Entry::Created(record.clone())))
}
