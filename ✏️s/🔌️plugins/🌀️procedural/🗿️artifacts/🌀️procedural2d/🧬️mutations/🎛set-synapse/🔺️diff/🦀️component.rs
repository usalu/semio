use crate::artifacts::procedural2d::diff::Procedural2dDiff;
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use crate::artifacts::procedural2d::mutations::Procedural2dMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &Procedural2dMutation, base: &Procedural2dSnapshot) -> Procedural2dDiff {
    <Procedural2dMutation as protocol::Mutation<Procedural2dSnapshot>>::diff(mutation, base)
}
