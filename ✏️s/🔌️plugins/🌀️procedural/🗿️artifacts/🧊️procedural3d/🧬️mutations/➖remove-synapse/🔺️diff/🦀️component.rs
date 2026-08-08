use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &Procedural3dMutation, base: &Procedural3dSnapshot) -> Procedural3dDiff {
    <Procedural3dMutation as protocol::Mutation<Procedural3dSnapshot>>::diff(mutation, base)
}
