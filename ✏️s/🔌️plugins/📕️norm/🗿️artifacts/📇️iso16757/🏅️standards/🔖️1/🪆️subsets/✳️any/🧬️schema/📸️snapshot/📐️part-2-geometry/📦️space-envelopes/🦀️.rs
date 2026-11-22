//! 📦️ Finite three-dimensional envelopes for ISO 16757-2 §5.3.

/// 📐️ Volume of a finite ordered box, available only if every extent and the volume are positive and finite.
pub fn envelope_volume(min: [f64; 3], max: [f64; 3]) -> Option<f64> {
    let extents = std::array::from_fn::<_, 3, _>(|axis| max[axis] - min[axis]);
    if !extents.iter().all(|extent| extent.is_finite() && *extent > 0.0) {
        return None;
    }
    let volume: f64 = extents.into_iter().product();
    (volume.is_finite() && volume > 0.0).then_some(volume)
}

/// 🧭️ Checks all lower and upper face coordinates, allowing exact boundary contact.
pub fn contains_box(outer_min: [f64; 3], outer_max: [f64; 3], min: [f64; 3], max: [f64; 3]) -> bool {
    envelope_volume(outer_min, outer_max).is_some()
        && envelope_volume(min, max).is_some()
        && (0..3).all(|axis| outer_min[axis] <= min[axis] && max[axis] <= outer_max[axis])
}
