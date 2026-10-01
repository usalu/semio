//! 🔺️ `change-storeys` diff — sets the document's storey count; the same value is a `mutation.no-op`.

use super::ChangeStoreys;
use crate::{En1996Diff, En1996Snapshot};

pub fn diff(payload: &ChangeStoreys, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if base.storeys == payload.new_storeys {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "The storey count already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { storeys: Some(payload.new_storeys), ..Default::default() })
}
