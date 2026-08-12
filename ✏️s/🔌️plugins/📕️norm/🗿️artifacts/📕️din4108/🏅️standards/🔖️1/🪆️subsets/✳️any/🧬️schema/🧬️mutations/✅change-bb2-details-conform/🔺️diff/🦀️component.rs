//! 🔺️ `change-bb2-details-conform` — sparse diff construction.

use super::mutation::ChangeBb2DetailsConform;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeBb2DetailsConform, _base: &Din4108Snapshot) -> Din4108Diff {
    Din4108Diff { bb2_details_conform: Some(payload.new_bb2_details_conform), ..Default::default() }
}
//#endregion 🔖️Diff
