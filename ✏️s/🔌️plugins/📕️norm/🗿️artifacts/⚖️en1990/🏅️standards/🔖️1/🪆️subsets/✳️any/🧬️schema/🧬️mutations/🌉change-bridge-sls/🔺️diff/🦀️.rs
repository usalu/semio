//! 🔺️ `change-bridge-sls` sparse diff.

use super::ChangeBridgeSls;
use crate::diff::En1990Diff;
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeBridgeSls, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if &base.bridge_sls == &mutation.new_bridge_sls {
        return MutationOutcome::empty().warn("mutation.no-op", "bridge_sls already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        bridge_sls: Some(mutation.new_bridge_sls.clone()),
        ..En1990Diff::default()
    })
}
