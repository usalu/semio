//! 🔺️ `update-limits` — sparse diff construction.

use super::mutation::UpdateLimits;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &UpdateLimits, _base: &Vdi3805Snapshot) -> Vdi3805Diff {
    Vdi3805Diff { limits: Some(payload.new_limits), ..Default::default() }
}
//#endregion 🔖️Diff
