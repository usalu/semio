//! 🔺️ Sparse declarative delete-design-option diff.
use crate::standards::v1::subsets::any::schema::mutations::delete_design_option::DeleteDesignOption;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::{Entry, KeyedDelta};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};
pub fn diff(payload: &DeleteDesignOption, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.design_options.contains_key(&payload.id) { return MutationOutcome::refuse(OutcomeCode::TargetMissing, "Record is missing.", [payload.id.clone()]); }
    let memberships = base.element_options.iter().filter(|(_, membership)| membership.target == payload.id).map(|(id, _)| (id.clone(), Entry::Deleted)).collect::<std::collections::BTreeMap<_, _>>();
    let mut diff = ModelDiff::design_options(payload.id.clone(), Entry::Deleted);
    if !memberships.is_empty() { diff.element_options = Some(KeyedDelta(memberships)); }
    return MutationOutcome::new(diff);
}
