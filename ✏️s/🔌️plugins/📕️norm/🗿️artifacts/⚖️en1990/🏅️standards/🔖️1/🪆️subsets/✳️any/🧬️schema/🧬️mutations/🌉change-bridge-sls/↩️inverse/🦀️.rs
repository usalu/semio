//! ↩️ `change-bridge-sls` inverse.

use super::ChangeBridgeSls;
use crate::En1990Mutation;
use crate::En1990Snapshot;

pub fn inverse(mutation: &ChangeBridgeSls, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    let _ = mutation;
    vec![En1990Mutation::ChangeBridgeSls(ChangeBridgeSls { new_bridge_sls: base.bridge_sls.clone() })]
}
