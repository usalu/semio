use crate::artifacts::mathematical::{MathGeometry, MathProjection};

pub fn apply(projection: &mut MathProjection, geometry: &MathGeometry) {
    projection.geometry = geometry.clone();
}
