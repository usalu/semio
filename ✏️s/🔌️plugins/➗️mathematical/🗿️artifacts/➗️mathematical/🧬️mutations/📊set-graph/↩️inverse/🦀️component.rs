use crate::artifacts::mathematical::{mutations::MathMutation, MathProjection};

pub fn inverse(projection: &MathProjection) -> Vec<MathMutation> {
    vec![MathMutation::SetGraph { graph: projection.graph.clone() }]
}
