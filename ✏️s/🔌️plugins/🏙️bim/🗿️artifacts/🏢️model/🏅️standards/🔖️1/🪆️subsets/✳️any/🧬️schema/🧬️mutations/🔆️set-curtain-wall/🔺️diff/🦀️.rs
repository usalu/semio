//! 🔺️ Diff constructor for `SetCurtainWall`: a sparse curtain wall patch of exactly the named fields that differ from the base. Only the named fields are validated (axis, top,
//! base offset); a payload that changes nothing is a no-op. The type is set by `set-curtain-wall-type-of` and the grid by `set-curtain-wall-grid`.

use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::schema::mutations::wall_geometry::{flaw, top_flaw, Flaw};
use crate::standards::v1::subsets::any::schema::mutations::set_curtain_wall::SetCurtainWall;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{CurtainWall, ModelSnapshot};
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::{Entry, Patch};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationOutcome, OutcomeCode};

fn flaw_of(payload: &SetCurtainWall, base: &ModelSnapshot, wall: &CurtainWall) -> Option<Flaw> {
    payload.axis.as_ref().and_then(|axis| flaw(axis).map(|flaw| flaw.under(&["axis"])))
        .or_else(|| payload.base_offset.and_then(|offset| (!offset.is_finite()).then(|| Flaw::new(OutcomeCode::Invariant, &["base_offset"], "A base offset must be finite."))))
        .or_else(|| payload.top.as_ref().and_then(|top| top_flaw(base, &wall.storey, top)))
}

pub fn diff(payload: &SetCurtainWall, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(wall) = base.curtain_walls.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Curtain wall \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(flaw) = flaw_of(payload, base, wall) {
        return flaw.refuse();
    }
    let patch = payload.patch().minimal(wall);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Curtain wall \"{}\" already has these parameters.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::curtain_walls(payload.id.clone(), Entry::Patched(patch)))
}
