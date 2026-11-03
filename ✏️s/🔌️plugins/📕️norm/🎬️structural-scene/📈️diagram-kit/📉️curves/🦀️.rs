//! 📉️ Generic curves: uniform and break-point sampling, buckling reduction curves and standard fire curves.

use crate::diagram_kit::Xy;

/// 🔢️ Evenly spaced samples of `f` over `[lo, hi]` including both ends; `n` is the number of intervals.
pub fn sample(f: impl Fn(f64) -> f64, lo: f64, hi: f64, n: usize) -> Vec<Xy> {
    let n = n.max(1);
    (0..=n).map(|i| lo + (hi - lo) * i as f64 / n as f64).map(|x| [x, f(x)]).collect()
}

/// 🔢️ Samples of `f` over `[lo, hi]` that always contain every break abscissa, so kinks are exact.
pub fn sample_with_breaks(f: impl Fn(f64) -> f64, lo: f64, hi: f64, n: usize, breaks: &[f64]) -> Vec<Xy> {
    let mut xs: Vec<f64> = sample(|x| x, lo, hi, n).into_iter().map(|p| p[0]).collect();
    xs.extend(breaks.iter().copied().filter(|b| *b > lo && *b < hi));
    xs.sort_by(f64::total_cmp);
    xs.dedup_by(|a, b| (*a - *b).abs() < 1e-12);
    xs.into_iter().map(|x| [x, f(x)]).collect()
}

/// 🧮️ Logarithmically spaced values from `lo` to `hi` inclusive.
pub fn log_space(lo: f64, hi: f64, n: usize) -> Vec<f64> {
    let (a, b) = (lo.log10(), hi.log10());
    (0..=n.max(1)).map(|i| 10f64.powf(a + (b - a) * i as f64 / n.max(1) as f64)).collect()
}

/// 🏛️ EN 1993-1-1 6.3.1.2 reduction factor χ for relative slenderness `lambda` and imperfection factor `alpha`.
pub fn buckling_chi(lambda: f64, alpha: f64) -> f64 {
    if lambda <= 0.2 {
        return 1.0;
    }
    let phi = 0.5 * (1.0 + alpha * (lambda - 0.2) + lambda * lambda);
    (1.0 / (phi + (phi * phi - lambda * lambda).sqrt())).min(1.0)
}

/// 📉️ The buckling curve χ(λ̄) sampled over `[0, lambda_max]`, with the plateau end at 0.2 included.
pub fn buckling_curve(alpha: f64, lambda_max: f64, n: usize) -> Vec<Xy> {
    sample_with_breaks(|l| buckling_chi(l, alpha), 0.0, lambda_max, n, &[0.2])
}

/// 🔥️ Standard fire curve families of EN 1991-1-2 3.2.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FireCurve {
    Iso834,
    External,
    Hydrocarbon,
}

/// 🔥️ Gas temperature in °C after `minutes` for a standard fire curve.
pub fn fire_temperature(curve: FireCurve, minutes: f64) -> f64 {
    match curve {
        FireCurve::Iso834 => 20.0 + 345.0 * (8.0 * minutes + 1.0).log10(),
        FireCurve::External => 660.0 * (1.0 - 0.687 * (-0.32 * minutes).exp() - 0.313 * (-3.8 * minutes).exp()) + 20.0,
        FireCurve::Hydrocarbon => 1080.0 * (1.0 - 0.325 * (-0.167 * minutes).exp() - 0.675 * (-2.5 * minutes).exp()) + 20.0,
    }
}

/// 🔥️ The fire curve sampled over `[0, minutes]`.
pub fn fire_curve(curve: FireCurve, minutes: f64, n: usize) -> Vec<Xy> {
    sample(|t| fire_temperature(curve, t), 0.0, minutes, n)
}
