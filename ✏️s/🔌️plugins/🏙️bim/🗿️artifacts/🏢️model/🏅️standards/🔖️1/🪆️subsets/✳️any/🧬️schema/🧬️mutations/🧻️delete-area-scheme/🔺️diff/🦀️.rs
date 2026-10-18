//! 🔺️ Diff constructor for `DeleteAreaScheme`: the scheme leaves in one sparse diff together with its properties and classifications (see the shared
//! cascade). Nothing else references a scheme, so nothing blocks or cascades beyond that data.

use crate::standards::v1::subsets::any::schema::mutations::cascade;
use crate::standards::v1::subsets::any::schema::mutations::delete_area_scheme::DeleteAreaScheme;
use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteAreaScheme, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.area_schemes.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Area scheme \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Area scheme", Some(&payload.id))
}
