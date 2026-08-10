use crate::artifacts::mathematical::{mutations::MathematicalMutation, MathematicalSnapshot};

pub fn inverse(snapshot: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    vec![MathematicalMutation::SetGeometry { geometry: snapshot.geometry.clone() }]
}
