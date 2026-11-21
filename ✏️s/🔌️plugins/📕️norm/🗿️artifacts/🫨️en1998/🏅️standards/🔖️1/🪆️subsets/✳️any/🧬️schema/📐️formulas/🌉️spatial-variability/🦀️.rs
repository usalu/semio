//! 🌉️ EN 1998-2:2005+A2:2011 §3.3 simplified spatial variation, in one horizontal direction.

/// 📏️ Whether the simplified spatial-variability workflow applies to the continuous deck.
pub fn spatial_variability_required(length_m: f64, limit_m: f64, ground_types_vary: bool) -> Option<bool> {
    if ![length_m, limit_m].into_iter().all(|value| value.is_finite() && value > 0.0) {
        return None;
    }
    Some(ground_types_vary || length_m > limit_m)
}

/// 📈️ Set A multiplier and same-sign displacement magnitude of one support relative to the reference support.
pub fn spatial_set_a(design_displacement_m: f64, reference_distance_m: f64, decorrelation_m: f64) -> Option<(f64, f64)> {
    if ![design_displacement_m, reference_distance_m].into_iter().all(|value| value.is_finite() && value >= 0.0)
        || !decorrelation_m.is_finite() || decorrelation_m <= 0.0
    {
        return None;
    }
    let bound_m = design_displacement_m * std::f64::consts::SQRT_2;
    let strain = bound_m / decorrelation_m;
    let displacement = bound_m * (reference_distance_m / decorrelation_m).min(1.0);
    (strain.is_finite() && displacement.is_finite()).then_some((strain, displacement))
}

/// ↔️ Set B absolute displacement magnitude from the governing local strain and the adjacent spans.
pub fn spatial_set_b(left_span_m: f64, right_span_m: f64, governing_strain: f64, opposite_factor: f64) -> Option<f64> {
    if ![left_span_m, right_span_m, governing_strain, opposite_factor].into_iter().all(|value| value.is_finite() && value >= 0.0)
        || (left_span_m == 0.0 && right_span_m == 0.0)
    {
        return None;
    }
    let span = if left_span_m == 0.0 || right_span_m == 0.0 {
        left_span_m.max(right_span_m)
    } else {
        0.5 * left_span_m + 0.5 * right_span_m
    };
    let absolute = 0.5 * opposite_factor * governing_strain * span;
    absolute.is_finite().then_some(absolute)
}

/// 📊️ SRSS of the inertia response and the worse of the two separate pseudo-static effects.
pub fn spatial_combined_effect(inertia: f64, set_a: f64, set_b: f64) -> Option<f64> {
    if ![inertia, set_a, set_b].into_iter().all(f64::is_finite) {
        return None;
    }
    let demand = inertia.hypot(set_a.abs().max(set_b.abs()));
    demand.is_finite().then_some(demand)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bridge_spatial_variation_matches_neutral_cases() {
        for line in include_str!("../../../🧪️tests/🧫️fixtures/🌉️spatial-variability/🔣️.tsv").lines() {
            let (kind, values) = line.split_once('\t').unwrap();
            let fields: Vec<f64> = values.split('\t').map(|value| value.parse().unwrap()).collect();
            let (observed, expected): (Option<Vec<f64>>, &[f64]) = match (kind, fields.as_slice()) {
                ("a", [dg, length, lg, tail @ ..]) => (spatial_set_a(*dg, *length, *lg).map(|(e,d)| vec![e,d]), tail),
                ("b", [left, right, e, k, tail @ ..]) => (spatial_set_b(*left, *right, *e, *k).map(|v| vec![v]), tail),
                ("effect", [e, a, b, tail @ ..]) => (spatial_combined_effect(*e, *a, *b).map(|v| vec![v]), tail),
                ("required", [l, lim, variable, tail @ ..]) => (
                    spatial_variability_required(*l, *lim, *variable == 1.0).map(|v| vec![if v { 1.0 } else { 0.0 }]), tail
                ),
                _ => unreachable!(),
            };
            let observed = observed.unwrap_or_default();
            assert_eq!(observed.len(), expected.len(), "{line}");
            assert!(observed.iter().zip(expected).all(|(a,b)| (a-b).abs() < 1e-12), "{line}");
        }
    }
}
