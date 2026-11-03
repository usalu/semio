//! 🌉️ EN 1993-2 bridge fatigue: damage equivalence factors, orthotropic deck details, FLM3 verification (mm, N, MPa). See [EN 1993-2 §9](https://eurocodes.jrc.ec.europa.eu/).

/// 🌉️ λ2 = (Q_m1/Q_0)(N_obs/N_0)^(1/5) (road bridges, §9.5.2(4)).
pub fn lambda_2(qm1: f64, q0: f64, n_obs: f64, n0: f64) -> f64 {
    qm1 / q0 * (n_obs / n0).powf(0.2)
}

/// 🌉️ λ3 = (t_Ld/100)^(1/5) for a design life t_Ld in years.
pub fn lambda_3(t_ld: f64) -> f64 {
    (t_ld / 100.0).powf(0.2)
}

/// 🌉️ λ4 = [Σ (n_i/N_obs)(Q_mi/Q_m1)^5]^(1/5) over the further lanes, 1 for a single lane.
pub fn lambda_4(lanes: &[(f64, f64)], n_obs_1: f64, qm1: f64) -> f64 {
    (1.0 + lanes.iter().map(|(n, q)| n / n_obs_1 * (q / qm1).powi(5)).sum::<f64>()).powf(0.2)
}

/// 🌉️ Linear interpolation of a λ1 curve given as (critical length, λ1) pairs from the National Annex or Figure 9.5, clamped at the ends.
pub fn lambda_1_interpolated(table: &[(f64, f64)], length: f64) -> f64 {
    if length <= table[0].0 {
        return table[0].1;
    }
    for w in table.windows(2) {
        if length <= w[1].0 {
            return w[0].1 + (w[1].1 - w[0].1) * (length - w[0].0) / (w[1].0 - w[0].0);
        }
    }
    table[table.len() - 1].1
}

/// 🌉️ λ = min(λ1 λ2 λ3 λ4, λ_max) and Δσ_E,2 = λ Φ2 Δσ_p (§9.5.2).
pub fn delta_sigma_e2_bridge(l1: f64, l2: f64, l3: f64, l4: f64, lambda_max: f64, phi2: f64, delta_p: f64) -> f64 {
    (l1 * l2 * l3 * l4).min(lambda_max) * phi2 * delta_p
}

/// 🌉️ Fatigue utilisation γ_Ff Δσ_E,2/(Δσ_C/γ_Mf).
pub fn fatigue_utilisation(delta_e2: f64, delta_c: f64, gamma_ff: f64, gamma_mf: f64) -> f64 {
    gamma_ff * delta_e2 * gamma_mf / delta_c
}

/// 🌉️ Detail category ladder of EN 1993-1-9 Table 8.1, read from `ndp.en1993.1-9.category.c<NNN>` (unverified), strongest first.
pub fn category_ladder() -> [f64; 14] {
    [160, 140, 125, 112, 100, 90, 80, 71, 63, 56, 50, 45, 40, 36].map(|c| semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1993.1-9.category.c{c}")))
}

/// 🌉️ Largest category of the ladder not above `delta_c`.
pub fn category_at_or_below(delta_c: f64) -> Option<f64> {
    category_ladder().into_iter().find(|c| *c <= delta_c)
}

/// 🌉️ Orthotropic deck detail ids of EN 1993-1-9 Table 8.8, read from `ndp.en1993.1-9.deck.<id>` (unverified, from memory).
pub const ORTHOTROPIC_DETAIL_IDS: [&str; 5] = ["rib-to-deck-full-penetration", "rib-to-deck-partial-penetration-throat", "rib-to-deck-partial-penetration-plate", "rib-to-crossbeam-web-cutout", "rib-splice-butt-weld"];

/// 🌉️ Detail category of an orthotropic deck detail.
pub fn orthotropic_detail(id: &str) -> Option<f64> {
    let id = ORTHOTROPIC_DETAIL_IDS.iter().copied().find(|known| *known == id)?;
    Some(semio_s_plugin_norm_compliance::ndp_common(&format!("ndp.en1993.1-9.deck.{id}")))
}

/// 🌉️ Size effect k_s = (25/t)^0.2 for t > 25 mm (EN 1993-1-9 Table 8.1 notes).
pub fn size_effect(t: f64) -> f64 {
    let reference = semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.1-9.size-effect.t-ref");
    if t > reference { (reference / t).powf(semio_s_plugin_norm_compliance::ndp_common("ndp.en1993.1-9.size-effect.exponent")) } else { 1.0 }
}
