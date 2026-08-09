use crate::artifacts::mathematical::{MathematicalGraph, MathematicalSnapshot};

pub fn apply(snapshot: &mut MathematicalSnapshot, graph: &MathematicalGraph) {
    snapshot.graph = graph.clone();
}
