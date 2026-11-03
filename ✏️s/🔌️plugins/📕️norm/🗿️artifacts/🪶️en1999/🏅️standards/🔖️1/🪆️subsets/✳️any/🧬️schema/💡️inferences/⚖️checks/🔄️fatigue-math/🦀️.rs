//! 🔄️ EN 1999-1-3 fatigue: Annex J detail categories, bilinear S–N curve and Palmgren–Miner damage of stress-range spectra. SI units (Pa, cycles).
//! 🔬 Independent reference: the numpy implementation of `🧰️scripts/en1999-wave4/miner_ref.py`.

/// 📎️ N_C = 2e6 (Δσ_C reference), N_D = 5e6 (knee), N_L = 1e8 (cut-off).
pub const N_C: f64 = 2.0e6;
pub const N_D: f64 = 5.0e6;
pub const N_L: f64 = 1.0e8;

/// 📋 The detail categories of Annex J as entities: the category number is Δσ_C in MPa (unverified list; the slopes m1 and m2 are inputs of each fatigue detail).
pub const CATEGORY_MPA: [f64; 20] = [14.0, 16.0, 18.0, 20.0, 22.0, 25.0, 28.0, 32.0, 36.0, 40.0, 45.0, 50.0, 56.0, 63.0, 71.0, 80.0, 90.0, 100.0, 112.0, 125.0];

/// 📋 Detail category Δσ_C [Pa] (Annex J rows; unknown categories return `None`).
pub fn detail_category_delta_sigma_c(category: &str) -> Option<f64> {
    let token = category.trim().trim_end_matches("-weld");
    let v: f64 = token.parse().ok()?;
    CATEGORY_MPA.iter().find(|r| (**r - v).abs() < 1e-9).map(|r| r * 1.0e6)
}

/// 📋 The category ids (`"71"`) of every Annex J row, in ascending order.
pub fn category_ids() -> Vec<String> { CATEGORY_MPA.iter().map(|c| format!("{c:.0}")).collect() }


/// 📈 Knee stress Δσ_D = Δσ_C·(N_C/N_D)^(1/m1).
pub fn delta_sigma_d(delta_sigma_c: f64, m1: f64) -> f64 { delta_sigma_c * (N_C / N_D).powf(1.0 / m1.max(1e-6)) }

/// 📈 Cut-off stress Δσ_L = Δσ_D·(N_D/N_L)^(1/m2).
pub fn delta_sigma_l(delta_sigma_c: f64, m1: f64, m2: f64) -> f64 { delta_sigma_d(delta_sigma_c, m1) * (N_D / N_L).powf(1.0 / m2.max(1e-6)) }

/// 📈 Bi-linear S–N: Δσ_R(N) with m1 up to N_D, m2 up to N_L and the constant cut-off beyond.
pub fn fatigue_strength_pa(delta_sigma_c: f64, m1: f64, m2: f64, n_cycles: f64) -> f64 {
    if n_cycles <= 0.0 { return delta_sigma_c; }
    if n_cycles <= N_D { return delta_sigma_c * (N_C / n_cycles).powf(1.0 / m1.max(1e-6)); }
    let at_nd = delta_sigma_d(delta_sigma_c, m1);
    if n_cycles >= N_L { return delta_sigma_l(delta_sigma_c, m1, m2); }
    at_nd * (N_D / n_cycles).powf(1.0 / m2.max(1e-6))
}

/// 🔁 Endurance N(Δσ) of the S–N curve; infinite below the cut-off Δσ_L.
pub fn cycles_to_failure(delta_sigma: f64, delta_sigma_c: f64, m1: f64, m2: f64) -> f64 {
    if delta_sigma <= 0.0 { return f64::INFINITY; }
    let d = delta_sigma_d(delta_sigma_c, m1);
    if delta_sigma >= d { return N_C * (delta_sigma_c / delta_sigma).powf(m1); }
    if delta_sigma < delta_sigma_l(delta_sigma_c, m1, m2) { return f64::INFINITY; }
    N_D * (d / delta_sigma).powf(m2)
}

/// 🧮 Palmgren–Miner damage D = n/N of one block, evaluated at γ_Ff·γ_Mf·Δσ.
pub fn damage(delta_sigma: f64, n_cycles: f64, delta_sigma_c: f64, m1: f64, m2: f64, gamma_ff: f64, gamma_mf: f64) -> f64 {
    let n_allow = cycles_to_failure(delta_sigma * gamma_ff * gamma_mf, delta_sigma_c, m1, m2);
    if n_allow.is_infinite() { 0.0 } else { n_cycles / n_allow }
}

/// 🧮 Palmgren–Miner damage D = Σ n_i/N_i of a stress-range spectrum given as `(Δσ_i, n_i)` blocks.
pub fn spectrum_damage(blocks: &[(f64, f64)], delta_sigma_c: f64, m1: f64, m2: f64, gamma_ff: f64, gamma_mf: f64) -> f64 {
    blocks.iter().map(|(ds, n)| damage(*ds, *n, delta_sigma_c, m1, m2, gamma_ff, gamma_mf)).sum()
}

/// 📊 Equivalent constant-amplitude range Δσ_eq = (Σ n_i·Δσ_i^m / Σ n_i)^(1/m) of a spectrum for the slope `m`.
pub fn equivalent_range(blocks: &[(f64, f64)], m: f64) -> f64 {
    let n: f64 = blocks.iter().map(|b| b.1).sum();
    if n <= 0.0 || m <= 0.0 { return 0.0; }
    (blocks.iter().map(|(ds, k)| k * ds.powf(m)).sum::<f64>() / n).powf(1.0 / m)
}

/// 📈 `(N, Δσ_R)` samples of the S–N curve, log-spaced between `n_min` and `n_max`.
pub fn sn_samples(delta_sigma_c: f64, m1: f64, m2: f64, n_min: f64, n_max: f64, count: usize) -> Vec<(f64, f64)> {
    (0..=count).map(|i| n_min * (n_max / n_min).powf(i as f64 / count.max(1) as f64)).map(|n| (n, fatigue_strength_pa(delta_sigma_c, m1, m2, n))).collect()
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
