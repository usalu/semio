//! 🔺️ `change-application-type` — sparse diff construction.

use super::mutation::ChangeApplicationType;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeApplicationType, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if base.application_type == payload.new_application_type {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Application type already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { application_type: Some(payload.new_application_type.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
