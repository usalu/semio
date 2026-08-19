//! 🔺️ `change-system-losses-kwh` sparse diff construction — writes only `Din18599Diff.system_losses_kwh` from the payload.

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::change_system_losses_kwh::mutation::ChangeSystemLossesKwh;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeSystemLossesKwh, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if !payload.new_system_losses_kwh.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "System losses kwh must be a finite number.", Vec::<String>::new());
    }
    if base.system_losses_kwh == payload.new_system_losses_kwh {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "System losses kwh already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { system_losses_kwh: Some(payload.new_system_losses_kwh.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
