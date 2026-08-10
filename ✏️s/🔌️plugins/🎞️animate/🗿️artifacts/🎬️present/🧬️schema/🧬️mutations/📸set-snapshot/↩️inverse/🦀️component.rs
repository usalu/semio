//! 🃏set-deck `PresentMutation` inverse leaf.
use crate::artifacts::present::PresentSnapshot;
use crate::artifacts::present::mutations::PresentMutation;
use protocol::Mutation;

pub fn inverse(base: &PresentSnapshot, mutation: &PresentMutation) -> Vec<PresentMutation> {
    <PresentMutation as Mutation<PresentSnapshot>>::inverse(mutation, base)
}
