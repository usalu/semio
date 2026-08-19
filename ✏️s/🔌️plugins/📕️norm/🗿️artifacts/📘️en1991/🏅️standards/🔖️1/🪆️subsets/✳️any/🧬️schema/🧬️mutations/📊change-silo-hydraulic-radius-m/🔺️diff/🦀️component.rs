//! 🔺️ `change-silo-hydraulic-radius-m` — sparse diff construction.

use super::mutation::ChangeSiloHydraulicRadiusM;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeSiloHydraulicRadiusM, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_silo_hydraulic_radius_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Silo hydraulic radius m must be a finite number.", Vec::<String>::new());
    }
    if base.silo_hydraulic_radius_m == payload.new_silo_hydraulic_radius_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Silo hydraulic radius m already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { silo_hydraulic_radius_m: Some(payload.new_silo_hydraulic_radius_m.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
