//! ➕add-frame `LayoutMutation` inverse leaf.
use crate::artifacts::layout::LayoutDocument;
use crate::artifacts::layout::mutations::LayoutMutation;
use protocol::Mutation;

pub fn inverse(base: &LayoutDocument, mutation: &LayoutMutation) -> Vec<LayoutMutation> {
    <LayoutMutation as Mutation<LayoutDocument>>::inverse(mutation, base)
}
