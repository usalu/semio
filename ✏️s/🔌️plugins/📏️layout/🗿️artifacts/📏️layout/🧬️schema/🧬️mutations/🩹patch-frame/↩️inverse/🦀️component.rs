//! 🩹patch-frame `LayoutMutation` inverse leaf.
use crate::artifacts::layout::LayoutSnapshot;
use crate::artifacts::layout::mutations::LayoutMutation;
use protocol::Mutation;

pub fn inverse(base: &LayoutSnapshot, mutation: &LayoutMutation) -> Vec<LayoutMutation> {
    <LayoutMutation as Mutation<LayoutSnapshot>>::inverse(mutation, base)
}
