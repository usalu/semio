use crate::artifacts::procedural3d::Procedural3dSnapshot;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;

pub fn inverse(base: &Procedural3dSnapshot, mutation: &Procedural3dMutation) -> Vec<Procedural3dMutation> {
    <Procedural3dMutation as protocol::Mutation<Procedural3dSnapshot>>::inverse(mutation, base)
}
