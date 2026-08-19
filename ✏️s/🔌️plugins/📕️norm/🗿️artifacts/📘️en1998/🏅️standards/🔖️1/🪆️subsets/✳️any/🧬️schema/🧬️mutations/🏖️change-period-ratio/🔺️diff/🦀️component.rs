//! 🔺️ `change-period-ratio` sparse diff construction — writes only `En1998Diff.period_ratio` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_period_ratio::mutation::ChangePeriodRatio;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangePeriodRatio, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_period_ratio.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Period ratio must be a finite number, got {}.", payload.new_period_ratio), Vec::<String>::new());
    }
    if base.period_ratio == payload.new_period_ratio {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Period ratio is already {}.", payload.new_period_ratio));
    }
    protocol::MutationOutcome::new(En1998Diff { period_ratio: Some(payload.new_period_ratio.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
