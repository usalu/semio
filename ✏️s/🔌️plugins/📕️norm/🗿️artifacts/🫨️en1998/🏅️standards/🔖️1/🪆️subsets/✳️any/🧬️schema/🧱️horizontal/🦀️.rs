//! 🧱️ Horizontal elements of EN 1998-1: the in-plane resistance of diaphragms (4.4.2.5), the overturning stability of a building, the response spectrum of an accelerogram (3.2.3.1) and the elastic spectrum over period and damping. Pure functions of numbers; the checks and the scene call them so picture and report cannot disagree.

/// 🧱 In-plane diaphragm resistance, simply supported between two lateral load resisting systems under the uniformly distributed storey force (EN 1998-1 4.4.2.5, [EN 1998-1](https://eur-lex.europa.eu/)).
pub mod diaphragm {
    /// 📏 Depth of the shear section after the openings, `D_eff = max(D − Σ openings, 0)`.
    pub fn effective_depth_m(depth_m: f64, opening_width_m: f64) -> f64 {
        (depth_m - opening_width_m.max(0.0)).max(0.0)
    }

    /// 📐 Span and depth of a diaphragm: the entered values, else the plan extents along and across the action.
    pub fn extent_m(span_m: f64, depth_m: f64, plan_span_m: f64, plan_depth_m: f64) -> (f64, f64) {
        (if span_m > 0.0 { span_m } else { plan_span_m }, if depth_m > 0.0 { depth_m } else { plan_depth_m })
    }

    /// 🏋️ Design force of the diaphragm `F_d = γ_d F_i`.
    pub fn design_force_n(storey_force_n: f64, gamma_d: f64) -> f64 {
        gamma_d * storey_force_n
    }

    /// ✂️ Shear force at the position `x` of the span, `V(x) = F_d (1/2 − x/L)`.
    pub fn shear_at_n(design_force_n: f64, span_m: f64, x_m: f64) -> f64 {
        design_force_n * (0.5 - x_m / span_m.max(1e-9))
    }

    /// 🪢 Shear force at the supports, `V_Ed = F_d / 2`.
    pub fn support_shear_n(design_force_n: f64) -> f64 {
        0.5 * design_force_n
    }

    /// 🛡️ In-plane shear resistance `V_Rd = τ_Rd t D_eff` for `τ_Rd` in MPa.
    pub fn shear_resistance_n(shear_strength_mpa: f64, thickness_m: f64, depth_m: f64, opening_width_m: f64) -> f64 {
        shear_strength_mpa * 1.0e6 * thickness_m * effective_depth_m(depth_m, opening_width_m)
    }

    /// 📐 Thickness that makes `V_Rd` equal to the shear `V_Ed`; infinite without strength or section.
    pub fn required_thickness_m(shear_n: f64, shear_strength_mpa: f64, depth_m: f64, opening_width_m: f64) -> f64 {
        let capacity_per_metre = shear_strength_mpa * 1.0e6 * effective_depth_m(depth_m, opening_width_m);
        if capacity_per_metre > 0.0 { shear_n / capacity_per_metre } else { f64::INFINITY }
    }

    /// 💧 Shear flow along the support edge, `q = V / D_eff`.
    pub fn shear_flow_n_per_m(shear_n: f64, depth_m: f64, opening_width_m: f64) -> f64 {
        let d = effective_depth_m(depth_m, opening_width_m);
        if d > 0.0 { shear_n / d } else { f64::INFINITY }
    }

    /// 🔗 Chord force at the position `x`, `N(x) = M(x)/D` with `M(x) = F_d x (L − x) / (2 L)`.
    pub fn chord_force_at_n(design_force_n: f64, span_m: f64, depth_m: f64, x_m: f64) -> f64 {
        let l = span_m.max(1e-9);
        design_force_n * x_m * (l - x_m) / (2.0 * l * depth_m.max(1e-9))
    }

    /// 📍 Largest chord force at mid span, `N = F_d L / (8 D)`.
    pub fn chord_force_n(design_force_n: f64, span_m: f64, depth_m: f64) -> f64 {
        design_force_n * span_m / (8.0 * depth_m.max(1e-9))
    }

    /// 🔩 Chord resistance `N_Rd = A f_d` for `f_d` in MPa.
    pub fn chord_resistance_n(area_m2: f64, strength_mpa: f64) -> f64 {
        area_m2 * strength_mpa * 1.0e6
    }

    /// 🧮 Chord area that makes `N_Rd` equal to `N_Ed`; infinite without strength.
    pub fn required_chord_area_m2(chord_force_n: f64, strength_mpa: f64) -> f64 {
        if strength_mpa > 0.0 { chord_force_n / (strength_mpa * 1.0e6) } else { f64::INFINITY }
    }
}

/// 🏗️ Overturning stability about the edge of the base in the direction of the action.
pub mod overturning {
    /// 🔄 Overturning moment `M_ov = Σ F_i z_i` of the storey forces at the storey elevations.
    pub fn moment_nm(forces_n: &[f64], elevations_m: &[f64]) -> f64 {
        forces_n.iter().zip(elevations_m).map(|(f, z)| f * z).sum()
    }

    /// ⚖️ Stabilising moment `M_stab = γ G b / 2 + M_tie` of the permanent weight about the edge and the tie-downs.
    pub fn stabilising_moment_nm(permanent_weight_n: f64, base_width_m: f64, tie_down_moment_nm: f64, factor: f64) -> f64 {
        factor * permanent_weight_n * base_width_m / 2.0 + tie_down_moment_nm
    }

    /// 🏞️ Base width that makes `M_stab` equal to `M_ov`; infinite without weight.
    pub fn required_base_width_m(moment_nm: f64, permanent_weight_n: f64, tie_down_moment_nm: f64, factor: f64) -> f64 {
        if permanent_weight_n > 0.0 && factor > 0.0 { 2.0 * (moment_nm - tie_down_moment_nm).max(0.0) / (factor * permanent_weight_n) } else { f64::INFINITY }
    }

    /// 🪝 Tie-down moment that makes `M_stab` equal to `M_ov`.
    pub fn required_tie_down_moment_nm(moment_nm: f64, permanent_weight_n: f64, base_width_m: f64, factor: f64) -> f64 {
        (moment_nm - factor * permanent_weight_n * base_width_m / 2.0).max(0.0)
    }
}

/// 📈 Response spectrum of an accelerogram by the exact recursion for a piecewise linear ground acceleration (EN 1998-1 3.2.3.1.2 asks for the 5 % damped spectrum of every record).
pub mod time_history {
    use crate::part_1;

    /// 📊 Damping ratio of the reference spectrum.
    pub const REFERENCE_DAMPING: f64 = 0.05;

    /// 🔢 Number of periods of the range 0.2 T1 to 2 T1 the acceptance is judged at.
    pub const PERIOD_SAMPLES: usize = 19;

    /// 📜 Samples of a blank separated list; `None` when a token is not a finite number.
    pub fn parse_samples(text: &str) -> Option<Vec<f64>> {
        text.split_whitespace().map(|token| token.parse::<f64>().ok().filter(|v| v.is_finite())).collect()
    }

    /// 🔝 Peak ground acceleration of the samples.
    pub fn peak_acceleration(samples: &[f64]) -> f64 {
        samples.iter().fold(0.0_f64, |peak, a| peak.max(a.abs()))
    }

    /// 🎢 Peak relative displacement of the oscillator `u'' + 2 ξ ω u' + ω² u = −a_g(t)` at the sample instants; the ground acceleration is linear inside every step, so each step is solved exactly.
    pub fn peak_displacement_m(samples: &[f64], dt_s: f64, omega: f64, xi: f64) -> f64 {
        let wd = omega * (1.0 - xi * xi).sqrt();
        let (e, c, s) = ((-xi * omega * dt_s).exp(), (wd * dt_s).cos(), (wd * dt_s).sin());
        let (mut u, mut v, mut peak) = (0.0_f64, 0.0_f64, 0.0_f64);
        for pair in samples.windows(2) {
            let (f0, g) = (-pair[0], (pair[0] - pair[1]) / dt_s);
            let u_p = |t: f64| (f0 + g * t) / (omega * omega) - 2.0 * xi * g / omega.powi(3);
            let v_p = g / (omega * omega);
            let (u0, v0) = (u - u_p(0.0), v - v_p);
            let c2 = (v0 + xi * omega * u0) / wd;
            u = u_p(dt_s) + e * (u0 * c + c2 * s);
            v = v_p + e * ((-xi * omega * u0 + wd * c2) * c + (-xi * omega * c2 - wd * u0) * s);
            peak = peak.max(u.abs());
        }
        peak
    }

    /// 🎯 Pseudo-acceleration `S_a(T) = ω² max |u|` in m/s²; the zero period returns the peak ground acceleration.
    pub fn pseudo_acceleration(samples: &[f64], dt_s: f64, period_s: f64, xi: f64) -> f64 {
        if period_s <= 0.0 {
            return peak_acceleration(samples);
        }
        let omega = 2.0 * std::f64::consts::PI / period_s;
        omega * omega * peak_displacement_m(samples, dt_s, omega, xi)
    }

    /// 🗓️ The `count` periods from 0.2 T1 to 2 T1, equally spaced.
    pub fn periods_s(t1_s: f64, count: usize) -> Vec<f64> {
        let (lo, hi) = (0.2 * t1_s, 2.0 * t1_s);
        (0..count).map(|i| if count > 1 { lo + (hi - lo) * i as f64 / (count - 1) as f64 } else { lo }).collect()
    }

    /// 🎼 Spectrum of one record scaled by `scale` at the periods.
    pub fn record_spectrum(samples: &[f64], scale: f64, dt_s: f64, periods_s: &[f64]) -> Vec<f64> {
        let scaled: Vec<f64> = samples.iter().map(|a| a * scale).collect();
        periods_s.iter().map(|t| pseudo_acceleration(&scaled, dt_s, *t, REFERENCE_DAMPING)).collect()
    }

    /// 🧾 Judgement of a set of records against the elastic spectrum of the site.
    #[derive(Clone, Debug, PartialEq)]
    pub struct SetJudgement {
        pub records: usize,
        pub mean_zero_period: f64,
        pub zero_period_required: f64,
        pub lowest_mean_ratio: f64,
        pub lowest_period_s: f64,
        pub mean_spectrum: Vec<f64>,
        pub elastic: Vec<f64>,
        pub scale_to_accept: f64,
    }

    /// 🧾 Judges the records `(peak ground acceleration, spectrum)` against `a_g S` and the elastic spectrum `elastic` at `periods_s`: the mean zero-period acceleration must reach `zero_ratio a_g S` and the mean spectrum `min_ratio` times the elastic one at every period; `scale_to_accept` is the common factor on all records that reaches both.
    pub fn judge_set(records: &[(f64, Vec<f64>)], periods_s: &[f64], elastic: &[f64], a_g_s: f64, zero_ratio: f64, min_ratio: f64) -> SetJudgement {
        let n = records.len();
        let spectra: Vec<Vec<f64>> = records.iter().map(|r| r.1.clone()).collect();
        let mean_zero_period = if n > 0 { records.iter().map(|r| r.0).sum::<f64>() / n as f64 } else { 0.0 };
        let mean_spectrum: Vec<f64> = (0..elastic.len()).map(|p| if n > 0 { spectra.iter().map(|r| r.get(p).copied().unwrap_or(0.0)).sum::<f64>() / n as f64 } else { 0.0 }).collect();
        let acceptance = part_1::accelerogram_set_acceptance(&spectra, elastic, 0, min_ratio);
        let lowest_index = mean_spectrum.iter().zip(elastic).enumerate().filter(|(_, (_, e))| **e > 0.0).fold(None, |best: Option<(usize, f64)>, (i, (m, e))| match best { Some(b) if b.1 <= m / e => Some(b), _ => Some((i, m / e)) }).map_or(0, |b| b.0);
        let zero_period_required = zero_ratio * a_g_s;
        let k_zero = if mean_zero_period > 0.0 { zero_period_required / mean_zero_period } else { f64::INFINITY };
        let k_spectrum = if acceptance.lowest_mean_ratio > 0.0 { min_ratio / acceptance.lowest_mean_ratio } else { f64::INFINITY };
        SetJudgement { records: n, mean_zero_period, zero_period_required, lowest_mean_ratio: acceptance.lowest_mean_ratio, lowest_period_s: periods_s.get(lowest_index).copied().unwrap_or(0.0), mean_spectrum, elastic: elastic.to_vec(), scale_to_accept: k_zero.max(k_spectrum).max(1.0) }
    }
}

/// 🌋 Elastic response spectrum over period and damping, the ordinates of the surface the scene draws.
pub mod spectrum_surface {
    use crate::part_1;

    /// 🧗 Ordinate `S_e(T, ξ)` in m/s² with the damping correction `η(ξ)` (ξ in percent).
    pub fn ordinate(a_g: f64, s: f64, tb: f64, tc: f64, td: f64, xi_percent: f64, period_s: f64) -> f64 {
        part_1::elastic_response_spectrum_eta(a_g, s, tb, tc, td, period_s, part_1::damping_correction(xi_percent))
    }

    /// 🗺️ The grid `[damping][period]` of ordinates.
    pub fn grid(a_g: f64, s: f64, tb: f64, tc: f64, td: f64, periods_s: &[f64], dampings_percent: &[f64]) -> Vec<Vec<f64>> {
        dampings_percent.iter().map(|xi| periods_s.iter().map(|t| ordinate(a_g, s, tb, tc, td, *xi, *t)).collect()).collect()
    }
}
