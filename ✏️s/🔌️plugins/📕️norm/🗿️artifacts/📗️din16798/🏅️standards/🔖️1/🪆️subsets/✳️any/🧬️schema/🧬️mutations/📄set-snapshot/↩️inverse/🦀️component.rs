//! ↩️ Inverse for SetSnapshot on Din16798.
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

pub fn inverse(base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::SetSnapshot { snapshot: base.clone() }]
}
