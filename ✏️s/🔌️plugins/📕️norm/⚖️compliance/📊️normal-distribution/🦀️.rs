//! 📊️ Standard normal distribution and complementary error function for structural reliability checks.

const SQRT_PI: f64 = 1.772_453_850_905_516;

fn erf_series(x: f64) -> f64 {
    let (x2, mut term, mut sum) = (x * x, x, x);
    let mut n = 0.0;
    while n < 200.0 {
        n += 1.0;
        term *= -x2 / n;
        let add = term / (2.0 * n + 1.0);
        sum += add;
        if add.abs() < 1e-17 * sum.abs() {
            break;
        }
    }
    2.0 / SQRT_PI * sum
}

fn erfc_tail(x: f64) -> f64 {
    let mut t = x;
    for k in (1..=120).rev() {
        t = x + (k as f64 / 2.0) / t;
    }
    (-x * x).exp() / (SQRT_PI * t)
}

/// 📈️ The complementary error function, accurate in the far tails (relative 1e-12).
pub fn erfc(x: f64) -> f64 {
    if x < 0.0 {
        2.0 - erfc(-x)
    } else if x < 2.0 {
        1.0 - erf_series(x)
    } else {
        erfc_tail(x)
    }
}

/// 📈️ The standard normal distribution function Φ.
pub fn phi_cdf(x: f64) -> f64 {
    0.5 * erfc(-x / std::f64::consts::SQRT_2)
}

/// 📈️ The standard normal density φ.
pub fn phi_pdf(x: f64) -> f64 {
    (-0.5 * x * x).exp() / (2.0 * std::f64::consts::PI).sqrt()
}

/// 📈️ The inverse standard normal distribution Φ⁻¹ (Acklam start, one Halley refinement).
pub fn phi_inv(p: f64) -> f64 {
    if !(p > 0.0 && p < 1.0) {
        return if p <= 0.0 { f64::NEG_INFINITY } else { f64::INFINITY };
    }
    const A: [f64; 6] = [-3.969_683_028_665_376e1, 2.209_460_984_245_205e2, -2.759_285_104_469_687e2, 1.383_577_518_672_69e2, -3.066_479_806_614_716e1, 2.506_628_277_459_239];
    const B: [f64; 5] = [-5.447_609_879_822_406e1, 1.615_858_368_580_409e2, -1.556_989_798_598_866e2, 6.680_131_188_771_972e1, -1.328_068_155_288_572e1];
    const C: [f64; 6] = [-7.784_894_002_430_293e-3, -3.223_964_580_411_365e-1, -2.400_758_277_161_838, -2.549_732_539_343_734, 4.374_664_141_464_968, 2.938_163_982_698_783];
    const D: [f64; 4] = [7.784_695_709_041_462e-3, 3.224_671_290_700_398e-1, 2.445_134_137_142_996, 3.754_408_661_907_416];
    let x = if p < 0.024_25 {
        let q = (-2.0 * p.ln()).sqrt();
        (((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5]) / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    } else if p > 1.0 - 0.024_25 {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();
        -(((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5]) / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    } else {
        let q = p - 0.5;
        let r = q * q;
        (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r + A[4]) * r + A[5]) * q / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r + B[4]) * r + 1.0)
    };
    let e = phi_cdf(x) - p;
    let u = e * (2.0 * std::f64::consts::PI).sqrt() * (0.5 * x * x).exp();
    x - u / (1.0 + 0.5 * x * u)
}

