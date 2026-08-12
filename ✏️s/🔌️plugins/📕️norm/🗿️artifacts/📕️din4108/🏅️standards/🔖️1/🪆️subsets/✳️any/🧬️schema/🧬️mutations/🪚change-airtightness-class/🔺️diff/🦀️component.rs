//! 🔺️ `change-airtightness-class` — sparse diff construction.

use super::mutation::ChangeAirtightnessClass;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeAirtightnessClass, _base: &Din4108Snapshot) -> Din4108Diff {
    Din4108Diff { airtightness_class: Some(payload.new_airtightness_class.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
