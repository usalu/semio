//! 🔺️ `change-declared-application-class` — sparse diff construction.

use super::ChangeDeclaredApplicationClass;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeDeclaredApplicationClass, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if base.declared_application_class == payload.new_declared_application_class {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Declared application class already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { declared_application_class: Some(payload.new_declared_application_class.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
