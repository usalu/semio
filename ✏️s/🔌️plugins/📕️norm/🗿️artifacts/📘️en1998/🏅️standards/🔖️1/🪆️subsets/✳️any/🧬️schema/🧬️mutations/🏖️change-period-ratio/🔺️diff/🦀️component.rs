//! 🔺️ `change-period-ratio` sparse diff construction — writes only `En1998Diff.period_ratio` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_period_ratio::mutation::ChangePeriodRatio;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangePeriodRatio, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { period_ratio: Some(payload.new_period_ratio.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
