use crate::artifacts::fem2d::Fem2dSnapshot;
use crate::artifacts::fem2d::mutations::Fem2dMutation;

pub fn inverse(base: &Fem2dSnapshot, mutation: &Fem2dMutation) -> Vec<Fem2dMutation> {
    <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::inverse(mutation, base)
}
