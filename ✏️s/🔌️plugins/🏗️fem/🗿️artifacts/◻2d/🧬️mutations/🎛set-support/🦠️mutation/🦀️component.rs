//! 🎛 Fem2d mutation — `SetSupport` apply delegate.
use crate::artifacts::fem2d::Fem2dSnapshot;
use crate::artifacts::fem2d::mutations::Fem2dMutation;

pub fn apply(snapshot: &mut Fem2dSnapshot, mutation: &Fem2dMutation) {
    crate::artifacts::fem2d::mutations::apply_fem2d_mutation(snapshot, mutation);
}
