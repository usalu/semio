//! 🌉 `change-bridge-sls` diff — replaces the whole collection: every base row leaves, every new row enters after the new row before it.

use super::ChangeBridgeSls;
use crate::diff::{En1990BridgeSlsAddition, En1990BridgeSlsDelta, En1990Diff};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeBridgeSls, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.bridge_sls == mutation.new_bridge_sls {
        return MutationOutcome::empty().warning("mutation.no-op", "bridge_sls already has this value.");
    }
    let removed = base.bridge_sls.iter().map(|row| row.id.clone()).collect();
    let added = mutation.new_bridge_sls.iter().enumerate().map(|(index, row)| En1990BridgeSlsAddition { after: index.checked_sub(1).map(|previous| mutation.new_bridge_sls[previous].id.clone()), row: row.clone() }).collect();
    MutationOutcome::new(En1990Diff { bridge_sls: En1990BridgeSlsDelta { removed, added, ..En1990BridgeSlsDelta::default() }, ..En1990Diff::default() })
}
