//! ➖ Fem3d mutation — `RemoveNode` apply delegate.
use crate::artifacts::fem3d::Fem3dDocument;
use crate::artifacts::fem3d::mutations::Fem3dMutation;

pub fn apply(projection: &mut Fem3dDocument, mutation: &Fem3dMutation) {
    crate::artifacts::fem3d::mutations::apply_fem3d_mutation(projection, mutation);
}
