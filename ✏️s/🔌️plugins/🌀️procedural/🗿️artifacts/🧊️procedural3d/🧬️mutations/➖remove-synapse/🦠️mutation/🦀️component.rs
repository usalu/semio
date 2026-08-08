//! ➖ Procedural3d mutation — `RemoveSynapse` apply delegate.
use crate::artifacts::procedural3d::Procedural3dDocument;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;

pub fn apply(projection: &mut Procedural3dDocument, mutation: &Procedural3dMutation) {
    crate::artifacts::procedural3d::mutations::apply_procedural3d_mutation(projection, mutation);
}
