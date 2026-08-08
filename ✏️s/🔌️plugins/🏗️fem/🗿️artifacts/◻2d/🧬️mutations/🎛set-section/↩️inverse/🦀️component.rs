use crate::artifacts::fem2d::Fem2dDocument;
use crate::artifacts::fem2d::mutations::Fem2dMutation;

pub fn inverse(base: &Fem2dDocument, mutation: &Fem2dMutation) -> Vec<Fem2dMutation> {
    <Fem2dMutation as protocol::Mutation<Fem2dDocument>>::inverse(mutation, base)
}
