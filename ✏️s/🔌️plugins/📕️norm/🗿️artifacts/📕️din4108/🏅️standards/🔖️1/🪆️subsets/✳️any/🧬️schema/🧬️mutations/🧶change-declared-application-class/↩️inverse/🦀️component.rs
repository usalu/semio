//! ↩️ `change-declared-application-class` — undo restores BASE's `declared_application_class`.

use super::mutation::ChangeDeclaredApplicationClass;
use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeDeclaredApplicationClass, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    vec![Din4108Mutation::ChangeDeclaredApplicationClass(ChangeDeclaredApplicationClass { new_declared_application_class: base.declared_application_class.clone() })]
}
//#endregion 🔖️Inverse
