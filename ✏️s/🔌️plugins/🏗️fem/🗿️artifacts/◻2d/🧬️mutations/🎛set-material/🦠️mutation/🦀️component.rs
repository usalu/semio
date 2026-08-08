//! 🎛 Fem2d mutation — `SetMaterial` apply delegate.
use crate::artifacts::fem2d::Fem2dDocument;
use crate::artifacts::fem2d::mutations::Fem2dMutation;

pub fn apply(projection: &mut Fem2dDocument, mutation: &Fem2dMutation) {
    crate::artifacts::fem2d::mutations::apply_fem2d_mutation(projection, mutation);
}
