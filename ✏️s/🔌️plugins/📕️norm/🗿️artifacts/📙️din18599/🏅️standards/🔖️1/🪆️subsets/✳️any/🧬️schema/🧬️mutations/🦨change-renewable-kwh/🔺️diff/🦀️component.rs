//! 🔺️ `change-renewable-kwh` sparse diff construction — writes only `Din18599Diff.renewable_kwh` from the payload.

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::change_renewable_kwh::mutation::ChangeRenewableKwh;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeRenewableKwh, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if !payload.new_renewable_kwh.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Renewable kwh must be a finite number.", Vec::<String>::new());
    }
    if base.renewable_kwh == payload.new_renewable_kwh {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Renewable kwh already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { renewable_kwh: Some(payload.new_renewable_kwh.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
