//! 📋set-tiles `PresentMutation` apply leaf.
use crate::artifacts::present::PresentSnapshot;
use crate::artifacts::present::mutations::PresentMutation;

pub fn apply(projection: &mut PresentSnapshot, mutation: &PresentMutation) {
    *projection = crate::artifacts::present::mutations::apply_present_mutation(projection, mutation);
}
