//! 🔺️ `change-service-class` sparse diff construction — writes only `En1995Diff.service_class` from the payload.

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::change_service_class::mutation::ChangeServiceClass;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeServiceClass, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
    if base.service_class == payload.new_service_class {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Service class already has this value.");
    }
    protocol::MutationOutcome::new(En1995Diff { service_class: Some(payload.new_service_class.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
