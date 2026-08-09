//! ↩️ Inverse for SetSnapshot on Din18599.
use crate::artifacts::din18599::mutations::Din18599Mutation;
use crate::artifacts::din18599::Din18599Snapshot;

pub fn inverse(base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::SetSnapshot { snapshot: base.clone() }]
}
