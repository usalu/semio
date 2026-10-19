//! 🔺️ Diff constructor for `DeleteSite`: the site, its buildings and everything inside them leave in one sparse diff together with everything that depends on them
//! (see the shared cascade), including the properties and classifications of every removed element. A storey that a
//! surviving element's top constraint still points at cannot cascade and is refused as `mutation.target-referenced`.

use crate::standards::v1::subsets::any::schema::mutations::delete_site::DeleteSite;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteSite, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.sites.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Site \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::outcome(base, std::slice::from_ref(&payload.id), "Site", Some(&payload.id))
}
