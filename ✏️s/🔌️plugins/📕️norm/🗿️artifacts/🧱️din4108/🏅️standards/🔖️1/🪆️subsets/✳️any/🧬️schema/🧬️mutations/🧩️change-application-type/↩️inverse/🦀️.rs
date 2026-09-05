//! ↩️ `change-application-type` — undo restores BASE's `application_type`.

use super::ChangeApplicationType;
use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeApplicationType, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    vec![Din4108Mutation::ChangeApplicationType(ChangeApplicationType { new_application_type: base.application_type.clone() })]
}
//#endregion 🔖️Inverse
