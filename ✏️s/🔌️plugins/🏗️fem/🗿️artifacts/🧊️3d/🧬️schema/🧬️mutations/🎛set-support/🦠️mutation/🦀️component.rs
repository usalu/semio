//! 🎛 Fem3d mutation — `SetSupport` apply delegate.
use crate::artifacts::fem3d::Fem3dSnapshot;
use crate::artifacts::fem3d::mutations::Fem3dMutation;

pub fn apply(snapshot: &mut Fem3dSnapshot, mutation: &Fem3dMutation) {
    crate::artifacts::fem3d::mutations::apply_fem3d_mutation(snapshot, mutation);
}
