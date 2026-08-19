//! 🔺️ `change-psi-times-l-sum` — sparse diff construction.

use super::mutation::ChangePsiTimesLSum;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangePsiTimesLSum, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if !payload.new_psi_times_l_sum.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Psi times l sum must be a finite number.", Vec::<String>::new());
    }
    if base.psi_times_l_sum == payload.new_psi_times_l_sum {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Psi times l sum already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { psi_times_l_sum: Some(payload.new_psi_times_l_sum.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
