//! 🔺️ Diff constructor for `DeleteSite`: the site, its buildings and everything inside them leave in one sparse diff together with everything that depends on them
//! (see the shared cascade), including the properties and classifications of every removed element. A storey that a
//! surviving element's top constraint still points at cannot cascade and is refused as `mutation.target-referenced`.

use super::super::cascade;
use super::DeleteSite;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteSite, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.sites.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Site \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Site", Some(&payload.id))
}
