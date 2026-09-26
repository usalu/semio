//! 🔺️ `change-usage` diff.

use super::ChangeUsage;
use crate::{Din4108Diff, Din4108Snapshot};

pub fn diff(payload: &ChangeUsage, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if base.usage == payload.new_usage {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "usage already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { usage: Some(payload.new_usage.clone()), ..Default::default() })
}
