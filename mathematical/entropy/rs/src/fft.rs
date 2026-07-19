//! 🌊 Hand-rolled discrete Fourier transform: iterative radix-2 Cooley-Tukey for power-of-two
//! lengths, Bluestein's chirp-z algorithm for arbitrary lengths (Welch segment lengths are
//! user-chosen and rarely powers of two), plus the standard analysis window functions.

use crate::EntropyError;
use std::ops::{Add, Mul, Sub};

// #region 🔖Complex
/// 🌊 A minimal complex number, `re + i*im`.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub const fn zero() -> Self {
        Self { re: 0.0, im: 0.0 }
    }

    pub fn from_polar(magnitude: f64, angle: f64) -> Self {
        Self { re: magnitude * angle.cos(), im: magnitude * angle.sin() }
    }

    pub fn conj(self) -> Self {
        Self { re: self.re, im: -self.im }
    }

    pub fn norm_sq(self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    pub fn abs(self) -> f64 {
        self.norm_sq().sqrt()
    }

    pub fn arg(self) -> f64 {
        self.im.atan2(self.re)
    }

    pub fn scale(self, s: f64) -> Self {
        Self { re: self.re * s, im: self.im * s }
    }
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Complex {
        Complex { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}

impl Sub for Complex {
    type Output = Complex;
    fn sub(self, rhs: Complex) -> Complex {
        Complex { re: self.re - rhs.re, im: self.im - rhs.im }
    }
}

impl Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Complex {
        Complex { re: self.re * rhs.re - self.im * rhs.im, im: self.re * rhs.im + self.im * rhs.re }
    }
}
// #endregion 🔖Complex

// #region 🔖Radix2
fn bit_reverse_permute(data: &mut [Complex]) {
    let n = data.len();
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            data.swap(i, j);
        }
    }
}

/// 🌊 In-place iterative radix-2 Cooley-Tukey FFT. `n = data.len()` must be a power of two.
/// `inverse = true` computes the unnormalized inverse transform (caller divides by `n`).
fn fft_radix2_inplace(data: &mut [Complex], inverse: bool) {
    let n = data.len();
    if n <= 1 {
        return;
    }
    bit_reverse_permute(data);
    let sign = if inverse { 1.0 } else { -1.0 };
    let mut len = 2;
    while len <= n {
        let angle = sign * 2.0 * core::f64::consts::PI / len as f64;
        let wlen = Complex::from_polar(1.0, angle);
        let mut i = 0;
        while i < n {
            let mut w = Complex::new(1.0, 0.0);
            for k in 0..len / 2 {
                let u = data[i + k];
                let v = data[i + k + len / 2] * w;
                data[i + k] = u + v;
                data[i + k + len / 2] = u - v;
                w = w * wlen;
            }
            i += len;
        }
        len <<= 1;
    }
}

fn next_power_of_two(n: usize) -> usize {
    n.next_power_of_two()
}
// #endregion 🔖Radix2

// #region 🔖Bluestein
/// 🌊 Bluestein's chirp-z transform: reduces an arbitrary-length DFT to a power-of-two
/// convolution via the identity `n*k = (n^2 + k^2 - (n-k)^2) / 2`, so it applies exactly to any
/// length (used whenever `n` is not already a power of two).
fn fft_bluestein(data: &[Complex], inverse: bool) -> Vec<Complex> {
    let n = data.len();
    if n == 0 {
        return Vec::new();
    }
    let sign = if inverse { 1.0 } else { -1.0 };
    let chirp: Vec<Complex> = (0..n)
        .map(|k| {
            let angle = sign * core::f64::consts::PI * ((k as u128 * k as u128) % (2 * n as u128)) as f64 / n as f64;
            Complex::from_polar(1.0, angle)
        })
        .collect();

    let m = next_power_of_two(2 * n - 1);
    let mut a = vec![Complex::zero(); m];
    for k in 0..n {
        a[k] = data[k] * chirp[k];
    }
    let mut b = vec![Complex::zero(); m];
    b[0] = chirp[0].conj();
    for k in 1..n {
        let c = chirp[k].conj();
        b[k] = c;
        b[m - k] = c;
    }

    fft_radix2_inplace(&mut a, false);
    fft_radix2_inplace(&mut b, false);
    let mut conv: Vec<Complex> = a.iter().zip(b.iter()).map(|(&x, &y)| x * y).collect();
    fft_radix2_inplace(&mut conv, true);
    let inv_m = 1.0 / m as f64;
    for c in &mut conv {
        *c = c.scale(inv_m);
    }

    (0..n).map(|k| conv[k] * chirp[k]).collect()
}
// #endregion 🔖Bluestein

// #region 🔖Fft
/// 🌊 A reusable FFT plan for a fixed transform length `n`, dispatching to the fast radix-2 path
/// when `n` is a power of two and to Bluestein's algorithm otherwise. Caches nothing beyond `n`
/// itself (the radix-2 path recomputes twiddles per call; profiling has not shown this to matter
/// at the sample sizes entropy estimators use) but keeps a stable, testable, allocation-owning
/// API for callers that transform the same length repeatedly.
pub struct Fft {
    n: usize,
    power_of_two: bool,
}

impl Fft {
    pub fn new(n: usize) -> Self {
        Self { n, power_of_two: n.is_power_of_two() }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// 🌊 Forward DFT: `X_k = sum_j x_j * exp(-2*pi*i*j*k/n)`.
    pub fn forward(&self, input: &[Complex]) -> Vec<Complex> {
        assert_eq!(input.len(), self.n, "Fft::forward: input length must match plan length");
        if self.power_of_two {
            let mut data = input.to_vec();
            fft_radix2_inplace(&mut data, false);
            data
        } else {
            fft_bluestein(input, false)
        }
    }

    /// 🌊 Inverse DFT (normalized by `1/n`): `x_j = (1/n) sum_k X_k * exp(+2*pi*i*j*k/n)`.
    pub fn inverse(&self, input: &[Complex]) -> Vec<Complex> {
        assert_eq!(input.len(), self.n, "Fft::inverse: input length must match plan length");
        let mut data = if self.power_of_two {
            let mut d = input.to_vec();
            fft_radix2_inplace(&mut d, true);
            d
        } else {
            fft_bluestein(input, true)
        };
        let inv_n = 1.0 / self.n as f64;
        for c in &mut data {
            *c = c.scale(inv_n);
        }
        data
    }
}

/// 🌊 Real-input forward FFT, returning the one-sided spectrum (`n/2 + 1` bins, DC through
/// Nyquist) since a real signal's full spectrum is Hermitian-symmetric.
pub fn real_fft(input: &[f64]) -> Vec<Complex> {
    let n = input.len();
    let complex: Vec<Complex> = input.iter().map(|&x| Complex::new(x, 0.0)).collect();
    let full = Fft::new(n).forward(&complex);
    full.into_iter().take(n / 2 + 1).collect()
}

/// 🌊 Naive `O(n^2)` DFT, kept as the correctness oracle for [`Fft`] in tests — never used on the
/// hot path.
#[cfg(test)]
fn naive_dft(input: &[Complex], inverse: bool) -> Vec<Complex> {
    let n = input.len();
    let sign = if inverse { 1.0 } else { -1.0 };
    (0..n)
        .map(|k| {
            let mut sum = Complex::zero();
            for (j, &x) in input.iter().enumerate() {
                let angle = sign * 2.0 * core::f64::consts::PI * (j * k) as f64 / n as f64;
                sum = sum + x * Complex::from_polar(1.0, angle);
            }
            if inverse {
                sum.scale(1.0 / n as f64)
            } else {
                sum
            }
        })
        .collect()
}
// #endregion 🔖Fft

// #region 🔖Window
/// 🌊 Analysis window function family for spectral estimation.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum WindowKind {
    Rectangular,
    Hann,
    Hamming,
    Blackman,
    BlackmanHarris,
    /// 🌊 Kaiser window with shape parameter `beta` (larger = narrower mainlobe, higher sidelobes
    /// suppressed less).
    Kaiser(f64),
    /// 🌊 Tukey (tapered cosine) window with taper fraction `alpha` in `[0, 1]`.
    Tukey(f64),
}

/// 🌊 Modified zeroth-order Bessel function `I0(x)`, needed by the Kaiser window, via its power
/// series (converges rapidly for the `x` ranges Kaiser windows use).
fn bessel_i0(x: f64) -> f64 {
    let mut term = 1.0_f64;
    let mut sum = 1.0_f64;
    let half_x_sq = (x / 2.0) * (x / 2.0);
    for k in 1..64 {
        term *= half_x_sq / (k as f64 * k as f64);
        sum += term;
        if term < sum * 1e-17 {
            break;
        }
    }
    sum
}

/// 🌊 Samples a length-`n` window of the given kind.
pub fn window(kind: WindowKind, n: usize) -> Vec<f64> {
    if n == 0 {
        return Vec::new();
    }
    if n == 1 {
        return vec![1.0];
    }
    let nf = (n - 1) as f64;
    match kind {
        WindowKind::Rectangular => vec![1.0; n],
        WindowKind::Hann => {
            (0..n).map(|i| 0.5 - 0.5 * (2.0 * core::f64::consts::PI * i as f64 / nf).cos()).collect()
        }
        WindowKind::Hamming => {
            (0..n).map(|i| 0.54 - 0.46 * (2.0 * core::f64::consts::PI * i as f64 / nf).cos()).collect()
        }
        WindowKind::Blackman => (0..n)
            .map(|i| {
                let x = 2.0 * core::f64::consts::PI * i as f64 / nf;
                0.42 - 0.5 * x.cos() + 0.08 * (2.0 * x).cos()
            })
            .collect(),
        WindowKind::BlackmanHarris => (0..n)
            .map(|i| {
                let x = 2.0 * core::f64::consts::PI * i as f64 / nf;
                0.358_75 - 0.488_29 * x.cos() + 0.141_28 * (2.0 * x).cos() - 0.011_68 * (3.0 * x).cos()
            })
            .collect(),
        WindowKind::Kaiser(beta) => {
            let i0_beta = bessel_i0(beta);
            (0..n)
                .map(|i| {
                    let ratio = (2.0 * i as f64 / nf) - 1.0;
                    let arg = beta * (1.0 - ratio * ratio).max(0.0).sqrt();
                    bessel_i0(arg) / i0_beta
                })
                .collect()
        }
        WindowKind::Tukey(alpha) => {
            let alpha = alpha.clamp(0.0, 1.0);
            if alpha <= 0.0 {
                return window(WindowKind::Rectangular, n);
            }
            let taper = (alpha * nf / 2.0).floor() as usize;
            (0..n)
                .map(|i| {
                    if i < taper {
                        0.5 * (1.0 + (core::f64::consts::PI * (i as f64 / taper as f64 - 1.0)).cos())
                    } else if i >= n - taper {
                        let j = n - 1 - i;
                        0.5 * (1.0 + (core::f64::consts::PI * (j as f64 / taper as f64 - 1.0)).cos())
                    } else {
                        1.0
                    }
                })
                .collect()
        }
    }
}

/// 🌊 Validates that a window/segment length is nonzero, the one input check every windowed
/// caller needs before slicing.
pub fn validate_length(n: usize, what: &'static str) -> Result<(), EntropyError> {
    if n == 0 {
        return Err(EntropyError::EmptyInput { what });
    }
    Ok(())
}
// #endregion 🔖Window

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq_complex(a: Complex, b: Complex, tol: f64) -> bool {
        (a.re - b.re).abs() < tol && (a.im - b.im).abs() < tol
    }

    #[test]
    fn radix2_fft_matches_naive_dft() {
        for n in [2usize, 4, 8, 16, 32, 64] {
            let mut rng = crate::numeric::Xorshift64::new(n as u64);
            let input: Vec<Complex> = (0..n).map(|_| Complex::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5)).collect();
            let fast = Fft::new(n).forward(&input);
            let naive = naive_dft(&input, false);
            for (a, b) in fast.iter().zip(naive.iter()) {
                assert!(approx_eq_complex(*a, *b, 1e-9), "n={n}");
            }
        }
    }

    #[test]
    fn bluestein_fft_matches_naive_dft_for_arbitrary_lengths() {
        for n in [1usize, 3, 5, 6, 7, 11, 13, 17, 100, 101, 257] {
            let mut rng = crate::numeric::Xorshift64::new(n as u64 + 1);
            let input: Vec<Complex> = (0..n).map(|_| Complex::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5)).collect();
            let fast = Fft::new(n).forward(&input);
            let naive = naive_dft(&input, false);
            for (a, b) in fast.iter().zip(naive.iter()) {
                assert!(approx_eq_complex(*a, *b, 1e-6), "n={n} a={a:?} b={b:?}");
            }
        }
    }

    #[test]
    fn forward_then_inverse_roundtrips() {
        for n in [8usize, 15, 32, 100] {
            let mut rng = crate::numeric::Xorshift64::new(n as u64 + 99);
            let input: Vec<Complex> = (0..n).map(|_| Complex::new(rng.next_f64(), rng.next_f64())).collect();
            let plan = Fft::new(n);
            let forward = plan.forward(&input);
            let back = plan.inverse(&forward);
            for (a, b) in input.iter().zip(back.iter()) {
                assert!(approx_eq_complex(*a, *b, 1e-8), "n={n}");
            }
        }
    }

    #[test]
    fn parseval_theorem_holds() {
        let n = 32;
        let mut rng = crate::numeric::Xorshift64::new(3);
        let input: Vec<Complex> = (0..n).map(|_| Complex::new(rng.next_f64() - 0.5, 0.0)).collect();
        let spectrum = Fft::new(n).forward(&input);
        let time_energy: f64 = input.iter().map(|c| c.norm_sq()).sum();
        let freq_energy: f64 = spectrum.iter().map(|c| c.norm_sq()).sum::<f64>() / n as f64;
        assert!((time_energy - freq_energy).abs() < 1e-9);
    }

    #[test]
    fn real_fft_returns_one_sided_spectrum_length() {
        let signal: Vec<f64> = (0..16).map(|i| (i as f64).sin()).collect();
        let spectrum = real_fft(&signal);
        assert_eq!(spectrum.len(), 16 / 2 + 1);
    }

    #[test]
    fn window_functions_produce_expected_length_and_endpoints() {
        for kind in [
            WindowKind::Rectangular,
            WindowKind::Hann,
            WindowKind::Hamming,
            WindowKind::Blackman,
            WindowKind::BlackmanHarris,
            WindowKind::Kaiser(8.0),
            WindowKind::Tukey(0.5),
        ] {
            let w = window(kind, 64);
            assert_eq!(w.len(), 64);
            assert!(w.iter().all(|&v| v.is_finite() && v >= -1e-9));
        }
    }

    #[test]
    fn hann_window_endpoints_are_zero() {
        let w = window(WindowKind::Hann, 32);
        assert!(w[0].abs() < 1e-9);
        assert!(w[31].abs() < 1e-9);
    }

    #[test]
    fn rectangular_window_is_all_ones() {
        let w = window(WindowKind::Rectangular, 10);
        assert!(w.iter().all(|&v| (v - 1.0).abs() < 1e-12));
    }

    mod quick {
        use super::*;

        #[test]
        fn bluestein_matches_radix2_on_power_of_two_length() {
            let n = 64;
            let mut rng = crate::numeric::Xorshift64::new(77);
            let input: Vec<Complex> = (0..n).map(|_| Complex::new(rng.next_f64(), rng.next_f64())).collect();
            let radix2 = Fft::new(n).forward(&input);
            let bluestein = fft_bluestein(&input, false);
            for (a, b) in radix2.iter().zip(bluestein.iter()) {
                assert!(approx_eq_complex(*a, *b, 1e-6));
            }
        }

        #[test]
        fn large_prime_length_dft_matches_naive() {
            let n = 101; // prime, not near a power of two
            let mut rng = crate::numeric::Xorshift64::new(4242);
            let input: Vec<Complex> = (0..n).map(|_| Complex::new(rng.next_f64() - 0.5, 0.0)).collect();
            let fast = Fft::new(n).forward(&input);
            let naive = naive_dft(&input, false);
            for (a, b) in fast.iter().zip(naive.iter()) {
                assert!(approx_eq_complex(*a, *b, 1e-6));
            }
        }
    }
}
// #endregion 🔖Tests
