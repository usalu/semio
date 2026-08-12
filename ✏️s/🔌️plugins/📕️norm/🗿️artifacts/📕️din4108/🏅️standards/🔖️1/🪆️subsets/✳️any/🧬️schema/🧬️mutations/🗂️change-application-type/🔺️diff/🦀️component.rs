//! 🔺️ `change-application-type` — sparse diff construction.

use super::mutation::ChangeApplicationType;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeApplicationType, _base: &Din4108Snapshot) -> Din4108Diff {
    Din4108Diff { application_type: Some(payload.new_application_type.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
