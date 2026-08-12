//! 🔺️ `change-strict-mode` — sparse diff construction.

use super::mutation::ChangeStrictMode;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeStrictMode, _base: &Vdi3805Snapshot) -> Vdi3805Diff {
    Vdi3805Diff { strict_mode: Some(payload.new_strict_mode), ..Default::default() }
}
//#endregion 🔖️Diff
