//! 🔺️ `change-moisture-mu-exterior` — sparse diff construction.

use super::mutation::ChangeMoistureMuExterior;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeMoistureMuExterior, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if !payload.new_moisture_mu_exterior.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Moisture mu exterior must be a finite number.", Vec::<String>::new());
    }
    if base.moisture_mu_exterior == payload.new_moisture_mu_exterior {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Moisture mu exterior already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { moisture_mu_exterior: Some(payload.new_moisture_mu_exterior.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
