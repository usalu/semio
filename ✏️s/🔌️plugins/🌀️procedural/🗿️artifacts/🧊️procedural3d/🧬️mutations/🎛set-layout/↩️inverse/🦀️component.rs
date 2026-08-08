use crate::artifacts::procedural3d::Procedural3dDocument;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;

pub fn inverse(base: &Procedural3dDocument, mutation: &Procedural3dMutation) -> Vec<Procedural3dMutation> {
    <Procedural3dMutation as protocol::Mutation<Procedural3dDocument>>::inverse(mutation, base)
}
