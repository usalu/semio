use crate::artifacts::fem3d::Fem3dDocument;
use crate::artifacts::fem3d::mutations::Fem3dMutation;

pub fn inverse(base: &Fem3dDocument, mutation: &Fem3dMutation) -> Vec<Fem3dMutation> {
    <Fem3dMutation as protocol::Mutation<Fem3dDocument>>::inverse(mutation, base)
}
