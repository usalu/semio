//! 🔺️ Diff constructor for `SetColumnTilt`: a one-field column patch that assigns the tilt (absent in the payload clears it, the column is plumb again). A tilt leans by a
//! positive angle of at most 60 degrees towards a finite direction. The tilted solid, its volume and the joins follow by inference; the position stays the base point.

use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::schema::mutations::wall_geometry::tilt_flaw;
use crate::standards::v1::subsets::any::schema::mutations::set_column_tilt::SetColumnTilt;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::{Assigned, Entry};
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::building_elements::columns_beams_slabs::ColumnPatch;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetColumnTilt, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(column) = base.columns.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Column \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(flaw) = payload.tilt.as_ref().and_then(tilt_flaw) {
        return flaw.under(&["tilt"]).refuse();
    }
    if column.tilt == payload.tilt {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Column \"{}\" already has this tilt.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::columns(payload.id.clone(), Entry::Patched(ColumnPatch { tilt: Some(Assigned::new(payload.tilt)), ..Default::default() })))
}
