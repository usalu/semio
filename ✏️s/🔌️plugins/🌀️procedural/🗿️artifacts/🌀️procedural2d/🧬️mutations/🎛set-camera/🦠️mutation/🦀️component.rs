//! 🎛 Procedural2d mutation — `SetCamera` apply delegate.
use crate::artifacts::procedural2d::Procedural2dDocument;
use crate::artifacts::procedural2d::mutations::Procedural2dMutation;

pub fn apply(projection: &mut Procedural2dDocument, mutation: &Procedural2dMutation) {
    crate::artifacts::procedural2d::mutations::apply_procedural2d_mutation(projection, mutation);
}
