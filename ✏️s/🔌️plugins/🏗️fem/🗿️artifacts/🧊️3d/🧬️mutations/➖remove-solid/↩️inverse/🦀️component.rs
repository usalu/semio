use crate::artifacts::fem3d::Fem3dSnapshot;
use crate::artifacts::fem3d::mutations::Fem3dMutation;

pub fn inverse(base: &Fem3dSnapshot, mutation: &Fem3dMutation) -> Vec<Fem3dMutation> {
    <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::inverse(mutation, base)
}
