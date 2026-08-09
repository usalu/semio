//! ➖remove-frame `LayoutMutation` apply leaf.
use crate::artifacts::layout::LayoutSnapshot;
use crate::artifacts::layout::mutations::LayoutMutation;

pub fn apply(projection: &mut LayoutSnapshot, mutation: &LayoutMutation) {
    crate::artifacts::layout::mutations::apply_layout_mutation(projection, mutation);
}
