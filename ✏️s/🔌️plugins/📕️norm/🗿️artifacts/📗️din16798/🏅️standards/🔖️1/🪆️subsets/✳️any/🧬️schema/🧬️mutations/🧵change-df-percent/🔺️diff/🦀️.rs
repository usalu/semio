//! 🔺️ `change-df-percent` sparse diff construction — writes only `Din16798Diff.df_percent` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_df_percent::ChangeDfPercent;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeDfPercent, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_df_percent.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Daylight factor must be a finite number, got {}.", payload.new_df_percent), Vec::<String>::new());
    }
    if payload.new_df_percent < 0.0 || payload.new_df_percent > 100.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Daylight factor must be between 0 and 100 percent, got {}.", payload.new_df_percent), Vec::<String>::new());
    }
    if base.df_percent == payload.new_df_percent {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Daylight factor is already {}.", payload.new_df_percent));
    }
    protocol::MutationOutcome::new(Din16798Diff { df_percent: Some(payload.new_df_percent.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
