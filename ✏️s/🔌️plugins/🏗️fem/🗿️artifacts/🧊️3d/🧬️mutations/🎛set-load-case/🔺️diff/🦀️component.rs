use crate::artifacts::fem3d::diff::Fem3dDiff;
use crate::artifacts::fem3d::Fem3dDocument;
use crate::artifacts::fem3d::mutations::Fem3dMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &Fem3dMutation, base: &Fem3dDocument) -> Fem3dDiff {
    <Fem3dMutation as protocol::Mutation<Fem3dDocument>>::diff(mutation, base)
}
