//! ↩️ Inverse for SetSnapshot on Vdi3805.
use crate::artifacts::vdi3805::mutations::Vdi3805Mutation;
use crate::artifacts::vdi3805::Vdi3805Snapshot;

pub fn inverse(base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
    vec![Vdi3805Mutation::SetSnapshot { snapshot: base.clone() }]
}
