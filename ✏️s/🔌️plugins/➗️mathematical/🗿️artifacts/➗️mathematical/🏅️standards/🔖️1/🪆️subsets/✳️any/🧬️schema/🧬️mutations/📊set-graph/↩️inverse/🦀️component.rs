use crate::artifacts::mathematical::{mutations::MathematicalMutation, MathematicalSnapshot};

pub fn inverse(snapshot: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    vec![MathematicalMutation::SetGraph { graph: snapshot.graph.clone() }]
}
