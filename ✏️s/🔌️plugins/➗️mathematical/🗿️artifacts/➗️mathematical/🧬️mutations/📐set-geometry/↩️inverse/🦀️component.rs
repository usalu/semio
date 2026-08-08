use crate::artifacts::mathematical::{mutations::MathMutation, MathProjection};

pub fn inverse(projection: &MathProjection) -> Vec<MathMutation> {
    vec![MathMutation::SetGeometry { geometry: projection.geometry.clone() }]
}
