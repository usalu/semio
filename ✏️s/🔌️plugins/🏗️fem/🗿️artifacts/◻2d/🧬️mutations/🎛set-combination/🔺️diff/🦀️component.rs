use crate::artifacts::fem2d::diff::Fem2dDiff;
use crate::artifacts::fem2d::Fem2dDocument;
use crate::artifacts::fem2d::mutations::Fem2dMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &Fem2dMutation, base: &Fem2dDocument) -> Fem2dDiff {
    <Fem2dMutation as protocol::Mutation<Fem2dDocument>>::diff(mutation, base)
}
