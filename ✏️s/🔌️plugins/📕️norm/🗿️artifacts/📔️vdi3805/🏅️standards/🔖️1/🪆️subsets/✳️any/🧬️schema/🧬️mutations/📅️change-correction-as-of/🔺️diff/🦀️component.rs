//! 🔺️ `change-correction-as-of` — sparse diff construction.

use super::mutation::ChangeCorrectionAsOf;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeCorrectionAsOf, _base: &Vdi3805Snapshot) -> Vdi3805Diff {
    Vdi3805Diff { correction_as_of: Some(payload.new_correction_as_of), ..Default::default() }
}
//#endregion 🔖️Diff
