use crate::artifacts::mathematical::MathematicalSnapshot;

pub fn apply(snapshot: &mut MathematicalSnapshot, replacement: &MathematicalSnapshot) {
    *snapshot = replacement.clone();
}
