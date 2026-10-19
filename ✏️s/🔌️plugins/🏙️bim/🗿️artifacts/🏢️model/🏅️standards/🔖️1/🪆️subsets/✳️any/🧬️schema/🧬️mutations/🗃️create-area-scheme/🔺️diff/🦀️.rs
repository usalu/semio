//! 🔺️ Diff constructor for `CreateAreaScheme`: one created area scheme entry. The id must be free in every collection, every counted zone must
//! exist and a counted usage must not be blank (a blank usage could never match a space). An empty list counts everything, so it is valid.

use crate::standards::v1::subsets::any::schema::mutations::create_area_scheme::CreateAreaScheme;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateAreaScheme, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    let scheme = &payload.area_scheme;
    if let Some((code, message, field)) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::scheme_rule_issue(base, &scheme.usages, &scheme.zones) {
        return MutationOutcome::refuse(code, message, ["area_scheme", field]);
    }
    MutationOutcome::new(ModelDiff::area_schemes(payload.id.clone(), Entry::Created(scheme.clone())))
}
