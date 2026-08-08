//! 🃏set-deck `PresentMutation` apply leaf.
use crate::artifacts::present::PresentDeck;
use crate::artifacts::present::mutations::PresentMutation;

pub fn apply(projection: &mut PresentDeck, mutation: &PresentMutation) {
    *projection = crate::artifacts::present::mutations::apply_present_mutation(projection, mutation);
}
