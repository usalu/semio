//! 🔺️ Diff constructor for `DeleteCurtainWallType`: one deleted curtain wall type entry; refused while a curtain wall still uses the type.

use crate::standards::v1::subsets::any::schema::mutations::delete_curtain_wall_type::DeleteCurtainWallType;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteCurtainWallType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.curtain_wall_types.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Curtain wall type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if base.curtain_walls.values().any(|row| row.curtain_wall_type == payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Curtain wall type \"{}\" is still used by curtain walls.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::curtain_wall_types(payload.id.clone(), Entry::Deleted))
}
