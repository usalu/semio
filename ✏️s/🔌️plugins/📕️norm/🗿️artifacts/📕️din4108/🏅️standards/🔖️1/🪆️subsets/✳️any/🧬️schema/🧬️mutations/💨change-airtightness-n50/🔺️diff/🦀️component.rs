//! 🔺️ `change-airtightness-n50` — sparse diff construction.

use super::mutation::ChangeAirtightnessN50;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeAirtightnessN50, _base: &Din4108Snapshot) -> Din4108Diff {
    Din4108Diff { airtightness_n50: Some(payload.new_airtightness_n50), ..Default::default() }
}
//#endregion 🔖️Diff
