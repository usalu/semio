use crate::artifacts::procedural2d::diff::Procedural2dDiff;
use crate::artifacts::procedural2d::Procedural2dDocument;
use crate::artifacts::procedural2d::mutations::Procedural2dMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &Procedural2dMutation, base: &Procedural2dDocument) -> Procedural2dDiff {
    <Procedural2dMutation as protocol::Mutation<Procedural2dDocument>>::diff(mutation, base)
}
