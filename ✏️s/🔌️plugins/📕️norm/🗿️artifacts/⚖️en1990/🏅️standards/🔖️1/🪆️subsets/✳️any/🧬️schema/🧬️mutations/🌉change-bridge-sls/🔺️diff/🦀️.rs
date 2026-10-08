//! 🌉 `change-bridge-sls` diff — replaces the whole collection: every base row is removed back to front, then every new row is inserted in order.

use super::ChangeBridgeSls;
use crate::diff::En1990RowEdit as _;
use crate::diff::{En1990Diff, En1990BridgeSlsEdit};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeBridgeSls, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.bridge_sls == mutation.new_bridge_sls {
        return MutationOutcome::empty().warning("mutation.no-op", "bridge_sls already has this value.");
    }
    let removed = (0..base.bridge_sls.len()).rev().map(|index| En1990BridgeSlsEdit::remove(index, base.bridge_sls[index].id.clone()));
    let inserted = mutation.new_bridge_sls.iter().cloned().enumerate().map(|(index, row)| En1990BridgeSlsEdit::insert(index, row));
    MutationOutcome::new(En1990Diff { bridge_sls: removed.chain(inserted).collect(), ..En1990Diff::default() })
}
