//! ➕add-frame `LayoutMutation` apply leaf.
use crate::artifacts::layout::LayoutDocument;
use crate::artifacts::layout::mutations::LayoutMutation;

pub fn apply(projection: &mut LayoutDocument, mutation: &LayoutMutation) {
    crate::artifacts::layout::mutations::apply_layout_mutation(projection, mutation);
}
