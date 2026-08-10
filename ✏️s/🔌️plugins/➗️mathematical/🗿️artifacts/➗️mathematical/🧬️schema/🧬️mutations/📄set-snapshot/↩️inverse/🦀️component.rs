use crate::artifacts::mathematical::{mutations::MathematicalMutation, MathematicalSnapshot};

pub fn inverse(snapshot: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    vec![MathematicalMutation::SetSnapshot { snapshot: snapshot.clone() }]
}
