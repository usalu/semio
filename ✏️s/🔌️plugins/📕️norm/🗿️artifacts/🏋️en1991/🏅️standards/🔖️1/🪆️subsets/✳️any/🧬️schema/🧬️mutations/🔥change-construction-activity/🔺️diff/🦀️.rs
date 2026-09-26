//! Diff for `change-construction-activity`.
use super::ChangeConstructionActivity;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeConstructionActivity, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.construction_activity == payload.new_construction_activity {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { construction_activity: Some(payload.new_construction_activity.clone()), ..Default::default() })
}
