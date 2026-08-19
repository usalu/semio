//! 🔺️ `change-moisture-mu-interior` — sparse diff construction.

use super::mutation::ChangeMoistureMuInterior;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeMoistureMuInterior, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if !payload.new_moisture_mu_interior.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Moisture mu interior must be a finite number.", Vec::<String>::new());
    }
    if base.moisture_mu_interior == payload.new_moisture_mu_interior {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Moisture mu interior already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { moisture_mu_interior: Some(payload.new_moisture_mu_interior.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
