//! 🔺️ `change-silo-bulk-density-kn-m3` — sparse diff construction.

use super::ChangeSiloBulkDensityKnM3;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeSiloBulkDensityKnM3, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_silo_bulk_density_kn_m3.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Silo bulk density kn m3 must be a finite number.", Vec::<String>::new());
    }
    if base.silo_bulk_density_kn_m3 == payload.new_silo_bulk_density_kn_m3 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Silo bulk density kn m3 already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { silo_bulk_density_kn_m3: Some(payload.new_silo_bulk_density_kn_m3.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
