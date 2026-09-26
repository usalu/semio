//! 🔺️ `update-renewables` sparse diff.

use crate::diff::Din18599Diff;
use crate::mutations::update_renewables::UpdateRenewables;
use crate::Din18599Snapshot;

pub fn diff(payload: &UpdateRenewables, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {

    if base.renewables == payload.new_renewables {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "renewables already has this value.");
    }
    protocol::MutationOutcome::new(Din18599Diff { renewables: Some(payload.new_renewables.clone()), ..Default::default() })
}
