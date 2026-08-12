//! 🔺️ `change-df-percent` sparse diff construction — writes only `Din16798Diff.df_percent` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_df_percent::mutation::ChangeDfPercent;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeDfPercent, _base: &Din16798Snapshot) -> Din16798Diff {
    Din16798Diff { df_percent: Some(payload.new_df_percent.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
