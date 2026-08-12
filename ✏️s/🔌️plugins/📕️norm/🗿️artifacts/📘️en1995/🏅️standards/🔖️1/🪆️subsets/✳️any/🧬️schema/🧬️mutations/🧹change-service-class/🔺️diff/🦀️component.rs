//! 🔺️ `change-service-class` sparse diff construction — writes only `En1995Diff.service_class` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_service_class::mutation::ChangeServiceClass;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeServiceClass, _base: &En1995Snapshot) -> En1995Diff {
    En1995Diff { service_class: Some(payload.new_service_class.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
