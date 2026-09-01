//! 🔺️ `change-annual-limit-kwh` sparse diff construction — writes only `Din18599Diff.annual_limit_kwh` from the payload.

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::change_annual_limit_kwh::ChangeAnnualLimitKwh;
use crate::artifacts::din18599::Din18599Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeAnnualLimitKwh, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
    if !payload.new_annual_limit_kwh.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Annual limit kwh must be a finite number.", Vec::<String>::new());
    }
    if base.annual_limit_kwh == payload.new_annual_limit_kwh {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Annual limit kwh already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { annual_limit_kwh: Some(payload.new_annual_limit_kwh.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
