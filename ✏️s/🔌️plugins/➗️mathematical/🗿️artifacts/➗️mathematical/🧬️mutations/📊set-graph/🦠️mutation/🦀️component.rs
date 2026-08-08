use crate::artifacts::mathematical::{MathGraph, MathProjection};

pub fn apply(projection: &mut MathProjection, graph: &MathGraph) {
    projection.graph = graph.clone();
}
