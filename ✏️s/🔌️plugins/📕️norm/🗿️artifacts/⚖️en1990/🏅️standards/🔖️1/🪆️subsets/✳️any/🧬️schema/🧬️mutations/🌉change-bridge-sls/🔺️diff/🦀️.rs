//! 🌉 `change-bridge-sls` diff — replaces the whole collection: every base row leaves from its base index, every new row enters at its index.

use super::ChangeBridgeSls;
use crate::diff::{En1990BridgeSlsDelta, En1990BridgeSlsInsertion, En1990BridgeSlsRemoval, En1990Diff};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeBridgeSls, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.bridge_sls == mutation.new_bridge_sls {
        return MutationOutcome::empty().warning("mutation.no-op", "bridge_sls already has this value.");
    }
    let removed = base.bridge_sls.iter().enumerate().map(|(index, row)| En1990BridgeSlsRemoval { id: row.id.clone(), index }).collect();
    let inserted = mutation.new_bridge_sls.iter().enumerate().map(|(index, row)| En1990BridgeSlsInsertion { index, row: row.clone() }).collect();
    MutationOutcome::new(En1990Diff { bridge_sls: En1990BridgeSlsDelta { removed, inserted, ..En1990BridgeSlsDelta::default() }, ..En1990Diff::default() })
}
