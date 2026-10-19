//! 🔺️ Diff constructor for `SetCurtainPanelOverride`: a one-field override patch. A solid panel names an existing material, a door or window panel an existing type; the cell the
//! override addresses never changes (delete and create to address another cell).

use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::schema::mutations::wall_geometry::panel_flaw;
use crate::standards::v1::subsets::any::schema::mutations::set_curtain_panel_override::SetCurtainPanelOverride;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::building_elements::walls_and_curtain_walls::CurtainPanelOverridePatch;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetCurtainPanelOverride, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.curtain_panel_overrides.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Panel override \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(flaw) = panel_flaw(base, &payload.panel) {
        return flaw.under(&["panel"]).refuse();
    }
    if record.panel == payload.panel {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Panel override \"{}\" already holds this panel.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::curtain_panel_overrides(payload.id.clone(), Entry::Patched(CurtainPanelOverridePatch { panel: Some(payload.panel.clone()), ..Default::default() })))
}
