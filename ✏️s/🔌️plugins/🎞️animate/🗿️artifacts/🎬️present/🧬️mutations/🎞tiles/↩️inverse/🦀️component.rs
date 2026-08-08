//! 🎞tiles `PresentMutation` inverse leaf.
use crate::artifacts::present::PresentDeck;
use crate::artifacts::present::mutations::PresentMutation;
use protocol::Mutation;

pub fn inverse(base: &PresentDeck, mutation: &PresentMutation) -> Vec<PresentMutation> {
    <PresentMutation as Mutation<PresentDeck>>::inverse(mutation, base)
}
