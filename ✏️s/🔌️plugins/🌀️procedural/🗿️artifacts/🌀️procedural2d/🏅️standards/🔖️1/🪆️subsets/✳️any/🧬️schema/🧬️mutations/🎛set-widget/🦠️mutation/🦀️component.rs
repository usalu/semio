//! 🎛 Procedural2d mutation — `SetWidget` apply delegate.
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use crate::artifacts::procedural2d::mutations::Procedural2dMutation;

pub fn apply(projection: &mut Procedural2dSnapshot, mutation: &Procedural2dMutation) {
    crate::artifacts::procedural2d::mutations::apply_procedural2d_mutation(projection, mutation);
}
