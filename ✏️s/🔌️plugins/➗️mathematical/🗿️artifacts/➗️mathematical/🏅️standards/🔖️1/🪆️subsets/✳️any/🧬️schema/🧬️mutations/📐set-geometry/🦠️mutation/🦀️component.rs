use crate::artifacts::mathematical::{MathematicalGeometry, MathematicalSnapshot};

pub fn apply(snapshot: &mut MathematicalSnapshot, geometry: &MathematicalGeometry) {
    snapshot.geometry = geometry.clone();
}
