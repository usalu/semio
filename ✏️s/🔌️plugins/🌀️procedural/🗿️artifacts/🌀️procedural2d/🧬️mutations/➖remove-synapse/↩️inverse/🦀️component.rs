use crate::artifacts::procedural2d::Procedural2dSnapshot;
use crate::artifacts::procedural2d::mutations::Procedural2dMutation;

pub fn inverse(base: &Procedural2dSnapshot, mutation: &Procedural2dMutation) -> Vec<Procedural2dMutation> {
    <Procedural2dMutation as protocol::Mutation<Procedural2dSnapshot>>::inverse(mutation, base)
}
