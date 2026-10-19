//! 🔺️ Diff constructor for `DeleteWindowType`: one deleted window type entry together with the properties and classifications keyed by the type; refused while openings or curtain panels still use it.

use crate::standards::v1::subsets::any::schema::mutations::delete_window_type::DeleteWindowType;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::{CurtainPanel, OpeningKind};
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::{Entry, KeyedDelta};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteWindowType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.window_types.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Window type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if base.openings.values().any(|row| matches!(&row.kind, OpeningKind::Window { window_type } if *window_type == payload.id)) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Window type \"{}\" is still used by openings.", payload.id), [payload.id.clone()]);
    }
    let panel = |panel: &CurtainPanel| matches!(panel, CurtainPanel::Window { window_type } if *window_type == payload.id);
    if base.curtain_wall_types.values().any(|row| panel(&row.panel)) || base.curtain_panel_overrides.values().any(|row| panel(&row.panel)) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Window type \"{}\" is still used by curtain panels.", payload.id), [payload.id.clone()]);
    }
    let mut removal = semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::data_diff(base, &payload.id);
    removal.window_types = Some(KeyedDelta::one(payload.id.clone(), Entry::Deleted));
    MutationOutcome::new(removal)
}
