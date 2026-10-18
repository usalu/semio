//! 🔺️ Diff constructor for `DeleteBuilding`: the building, its storeys, grid lines and everything on them leave in one sparse diff together with everything that depends on them
//! (see the shared cascade), including the properties and classifications of every removed element. A storey that a
//! surviving element's top constraint still points at cannot cascade and is refused as `mutation.target-referenced`.

use crate::standards::v1::subsets::any::schema::mutations::cascade;
use crate::standards::v1::subsets::any::schema::mutations::delete_building::DeleteBuilding;
use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteBuilding, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.buildings.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Building \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Building", Some(&payload.id))
}
