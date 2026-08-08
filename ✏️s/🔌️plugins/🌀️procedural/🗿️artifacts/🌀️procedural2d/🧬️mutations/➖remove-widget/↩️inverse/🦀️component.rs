use crate::artifacts::procedural2d::Procedural2dDocument;
use crate::artifacts::procedural2d::mutations::Procedural2dMutation;

pub fn inverse(base: &Procedural2dDocument, mutation: &Procedural2dMutation) -> Vec<Procedural2dMutation> {
    <Procedural2dMutation as protocol::Mutation<Procedural2dDocument>>::inverse(mutation, base)
}
