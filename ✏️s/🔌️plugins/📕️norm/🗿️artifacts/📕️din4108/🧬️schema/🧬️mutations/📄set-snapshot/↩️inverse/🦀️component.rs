//! ↩️ Inverse for SetSnapshot on Din4108.
use crate::artifacts::din4108::mutations::Din4108Mutation;
use crate::artifacts::din4108::Din4108Snapshot;

pub fn inverse(base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    vec![Din4108Mutation::SetSnapshot { snapshot: base.clone() }]
}
