//! 🔺️ `change-declared-application-class` — sparse diff construction.

use super::mutation::ChangeDeclaredApplicationClass;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeDeclaredApplicationClass, _base: &Din4108Snapshot) -> Din4108Diff {
    Din4108Diff { declared_application_class: Some(payload.new_declared_application_class.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
