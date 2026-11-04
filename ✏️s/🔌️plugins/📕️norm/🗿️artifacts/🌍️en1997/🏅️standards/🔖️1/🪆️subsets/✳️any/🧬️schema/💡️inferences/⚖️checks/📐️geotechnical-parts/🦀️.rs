//! 📐️ EN 1997 geotechnical parts: the clause modules of EN 1997-1 and EN 1997-2 with their shared subject helpers.

use super::*;

pub fn subject_entity(id: &str, path: &str, en: &str, de: &str) -> SubjectRef {
    SubjectRef::new(id, path, loc(en, de))
}

pub mod part_1 {
    use crate::*;
    use crate::catalogue::{earth_pressure as ep, foundations as fnd, piles as pl, settlement as sl};

    /// 🪨️ Bulk unit weight γ = (G_s + S_r e) γ_w/(1 + e) of a soil from the degree of saturation, void ratio and grain density ratio; any consistent unit of γ_w.
    pub fn bulk_unit_weight(s_r: f64, e: f64, g_s: f64, gamma_w: f64) -> f64 { (g_s + s_r * e) * gamma_w / (1.0 + e) }

    /// 🪨️ Effective (buoyant) unit weight γ' = γ − γ_w below the water table.
    pub fn buoyant_unit_weight(s_r: f64, e: f64, g_s: f64, gamma_w: f64) -> f64 { bulk_unit_weight(s_r, e, g_s, gamma_w) - gamma_w }

    /// 📐️ Annex D bearing factor N_q = e^(π·tanφ)·tan²(45+φ/2).
    pub fn bearing_factor_n_q(phi_deg: f64) -> f64 { fnd::n_q(phi_deg) }

    /// 📐️ Annex D bearing factor N_c = (N_q − 1)·cot φ.
    pub fn bearing_factor_n_c(phi_deg: f64) -> f64 { fnd::n_c(phi_deg) }

    /// 📐️ Annex D bearing factor N_γ = 2·(N_q − 1)·tan φ.
    pub fn bearing_factor_n_gamma(phi_deg: f64) -> f64 { fnd::n_gamma(phi_deg) }

    /// 📐️ Effective width B′ = B − 2|e| (Annex D.1).
    pub fn effective_width(b: f64, eccentricity: f64) -> f64 {
        (b - 2.0 * eccentricity.abs()).max(1e-6)
    }

    /// 📐️ Effective unit weight [N/m³] below foundation base with GWL interpolation over width B (Annex D γ′).
    pub fn effective_gamma_below_pa_m(gamma: f64, gamma_prime: f64, embedment: f64, gwl: f64, width: f64) -> f64 {
        let base = embedment;
        let zone = width.max(0.1);
        if gwl >= base + zone {
            gamma
        } else if gwl <= base {
            gamma_prime.max(1.0)
        } else {
            let wet = ((base + zone) - gwl) / zone;
            gamma * (1.0 - wet) + gamma_prime.max(1.0) * wet
        }
    }

    /// 📐️ Design drained bearing resistance R_d [N] of Annex D.4 with design values tan φ_d = tan φ′/γφ, c_d = c′/γc and R/γR,v; 0 when the inputs leave Annex D (H beyond the limit or φ_d < 0.5°).
    pub fn design_bearing_resistance_with_params(
        phi_deg: f64, c_pa: f64, gamma_over_pa_m: f64, gamma_below_pa_m: f64, b: f64, l: f64, embedment: f64,
        e_b: f64, e_l: f64, h: f64, v: f64, base_inclination_deg: f64, p: &AnnexParams,
    ) -> f64 {
        let input = fnd::BearingIn {
            b, l, d: embedment, alpha_deg: base_inclination_deg, v, hb: h, hl: 0.0, e_b, e_l,
            phi_deg: (phi_deg.to_radians().tan() / p.gamma_phi).atan().to_degrees(),
            c: c_pa / p.gamma_c,
            gamma_below: gamma_below_pa_m / p.gamma_gamma,
            q_over: gamma_over_pa_m / p.gamma_gamma * embedment,
            include_depth: false,
        };
        fnd::bearing_drained(&input).map(|o| o.r / p.gamma_r_v).unwrap_or(0.0)
    }

    /// 📐️ Passive earth-pressure force E_p,k [N] on the front face (curved failure surface, wall friction magnitude δ).
    pub fn passive_earth_force_n(phi_deg: f64, gamma_pa_m: f64, embedment: f64, length: f64, wall_friction_deg: f64) -> f64 {
        if embedment <= 0.0 || length <= 0.0 {
            return 0.0;
        }
        0.5 * gamma_pa_m * embedment * embedment * ep::kp_curved(phi_deg, wall_friction_deg) * length
    }

    /// 📐️ Design sliding resistance [N]: (c_d·A + V·tan φ_d)/γR,h plus the secured share of E_p,k over γR,e.
    pub fn sliding_resistance_with_params(
        phi_deg: f64, c_pa: f64, v_n: f64, a: f64, p: &AnnexParams,
        embedment: f64, length: f64, gamma_pa_m: f64, permanent_embedment: bool,
    ) -> f64 {
        let phi_d = (phi_deg.to_radians().tan() / p.gamma_phi).atan().to_degrees();
        let base = c_pa / p.gamma_c * a + v_n * phi_deg.to_radians().tan() / p.gamma_phi;
        let passive = if permanent_embedment && embedment > 0.0 {
            0.5 * passive_earth_force_n(phi_d, gamma_pa_m, embedment, length, 0.0) / p.gamma_r_e
        } else {
            0.0
        };
        base / p.gamma_r_h + passive
    }

    /// 📐️ Layer-wise stress-strain settlement [m] (EN 1997-1 §6.6, DIN 4019): Boussinesq stress under the centre of the rectangle in sublayers of at most 0.5 m, cut off at σz ≤ 0.2·σ′v0; returns (s, governing layer id).
    pub fn settlement_oedometric_m(layers: &[SoilLayer], width: f64, length: f64, embedment: f64, q_sls_pa: f64) -> (f64, String) {
        let (s, id, _) = settlement_oedometric_with_gwl(layers, width, length, embedment, q_sls_pa, f64::INFINITY);
        (s, id)
    }

    fn effective_stress_at(layers: &[SoilLayer], depth: f64, gwl: f64) -> f64 {
        let mut sigma = 0.0;
        for layer in layers {
            let top = layer.depth_top.min(depth);
            let bottom = layer.depth_bottom.min(depth);
            if bottom <= top {
                continue;
            }
            let dry = (gwl.min(bottom) - top).max(0.0).min(bottom - top);
            sigma += layer.gamma * dry + layer.gamma_prime * (bottom - top - dry);
        }
        sigma
    }

    /// 📐️ Stress-strain settlement with GWL; returns `(s, governing_layer_id, governing_layer_contribution)`.
    pub fn settlement_oedometric_with_gwl(
        layers: &[SoilLayer], width: f64, length: f64, embedment: f64, q_sls_pa: f64, gwl: f64,
    ) -> (f64, String, f64) {
        let mut s = 0.0;
        let mut best_id = String::new();
        let mut best_contrib = 0.0;
        let mut cut = false;
        for layer in layers {
            if cut {
                break;
            }
            let z_top = (layer.depth_top - embedment).max(0.0);
            let z_bot = (layer.depth_bottom - embedment).max(0.0);
            if z_bot <= z_top + 1e-12 {
                continue;
            }
            let parts = ((z_bot - z_top) / 0.5).ceil().max(1.0) as usize;
            let dz = (z_bot - z_top) / parts as f64;
            let e = layer.oedometric_modulus.max(1.0);
            let mut contrib = 0.0;
            for k in 0..parts {
                let z_mid = z_top + (k as f64 + 0.5) * dz;
                let sigma_z = sl::sigma_z_rect_centre(q_sls_pa, width, length.max(width), z_mid.max(1e-3));
                if sigma_z <= 0.2 * effective_stress_at(layers, embedment + z_mid, gwl) {
                    cut = true;
                    break;
                }
                contrib += sigma_z / e * dz;
            }
            s += contrib;
            if contrib >= best_contrib {
                best_contrib = contrib;
                best_id = layer.id.clone();
            }
        }
        if best_id.is_empty() {
            if let Some(first) = layers.first() {
                best_id = first.id.clone();
            }
        }
        (s, best_id, best_contrib)
    }

    pub fn settlement_m(layers: &[SoilLayer], width: f64, q_pa: f64, _poisson: f64) -> f64 {
        settlement_oedometric_m(layers, width, width, 0.0, q_pa).0
    }

    pub fn shaft_resistance_n(alpha_s: f64, d: f64, q_s_pa: f64, l: f64) -> f64 {
        alpha_s * std::f64::consts::PI * d * q_s_pa * l
    }

    pub fn base_resistance_n(q_b_pa: f64, a_base: f64) -> f64 {
        q_b_pa * a_base
    }

    /// 📐️ Ground-test correlation factors (ξ3, ξ4) of EN 1997-1 Table A.10.
    pub fn pile_correlation_factors(n_profiles: u32) -> (f64, f64) {
        pl::xi_ground_tests(n_profiles)
    }

    pub fn pile_characteristic_resistance_n(mean: f64, min: f64, n: u32) -> f64 {
        let (xi3, xi4) = pile_correlation_factors(n);
        (mean / xi3).min(min / xi4)
    }

    pub fn pile_design_resistance_with_params(r_b_k: f64, r_s_k: f64, p: &AnnexParams) -> f64 {
        r_b_k / p.gamma_b + r_s_k / p.gamma_s
    }

    /// 📐️ Undrained Annex D.3 design bearing resistance [N]; 0 when H exceeds A′·c_u.
    pub fn undrained_bearing_resistance_n(c_u: f64, gamma: f64, b: f64, l: f64, embedment: f64, e_b: f64, h: f64, v: f64, p: &AnnexParams) -> f64 {
        let cu_d = c_u / p.gamma_cu;
        let input = fnd::BearingIn { b, l, d: embedment, alpha_deg: 0.0, v, hb: h, hl: 0.0, e_b, e_l: 0.0, phi_deg: 0.0, c: 0.0, gamma_below: gamma, q_over: gamma / p.gamma_gamma * embedment, include_depth: false };
        fnd::bearing_undrained(&input, cu_d).map(|o| o.r / p.gamma_r_v).unwrap_or(0.0)
    }

    /// 📐️ Short-term undrained sliding R_d = A·c_u / (γcu·γR,h) [N].
    pub fn undrained_sliding_resistance_n(c_u: f64, a: f64, p: &AnnexParams) -> f64 {
        (c_u / p.gamma_cu) * a / p.gamma_r_h
    }

    /// 📐️ Immediate settlement of a flexible rectangle centre on the elastic half-space, s = 4·p·(B/2)·(1−ν²)/E·I(L/B) (Love's corner solution, Annex F.2 form s = p·b·f/E_m); the influence is symmetric in the sides, so the longer side may be given first.
    pub fn elastic_settlement_m(q_pa: f64, width: f64, length: f64, nu: f64, e_pa: f64) -> f64 {
        let b = width.max(0.1);
        sl::displacement_rect_at(q_pa, b, length.max(b), 0.0, 0.0, nu, e_pa)
    }

    /// 📐️ Equivalent half-space of the ground below a base at `embedment` over the depth 2B: `(ν, E_m, governing layer id)` with E_m = Σh/Σ(h/E) and the mean ν; `None` when no layer lies within the depth.
    pub fn elastic_equivalent(layers: &[SoilLayer], width: f64, embedment: f64) -> Option<(f64, f64, String)> {
        let limit = 2.0 * width.max(0.1);
        let (mut sum_h, mut sum_h_over_e, mut sum_nu, mut best, mut best_id) = (0.0, 0.0, 0.0, 0.0, String::new());
        for layer in layers {
            let top = (layer.depth_top - embedment).max(0.0);
            if top >= limit {
                continue;
            }
            let h = ((layer.depth_bottom - embedment).max(top).min(limit) - top).max(0.0);
            if h < 1e-9 {
                continue;
            }
            let compliance = h / layer.oedometric_modulus.max(1.0);
            sum_h += h;
            sum_h_over_e += compliance;
            sum_nu += layer.poisson_ratio * h;
            if compliance >= best {
                best = compliance;
                best_id = layer.id.clone();
            }
        }
        (sum_h >= 1e-9).then(|| ((sum_nu / sum_h).clamp(0.0, 0.49), sum_h / sum_h_over_e, best_id))
    }

    /// 📐️ Layered immediate settlement with the equivalent modulus E_m = Σh/Σ(h/E) and mean ν over the depth 2B; returns (s, governing layer id).
    pub fn elastic_settlement_layered(layers: &[SoilLayer], width: f64, length: f64, embedment: f64, q_pa: f64) -> (f64, String) {
        match elastic_equivalent(layers, width, embedment) {
            Some((nu, e_m, id)) => (elastic_settlement_m(q_pa, width, length, nu, e_m), id),
            None => (0.0, layers.first().map(|l| l.id.clone()).unwrap_or_default()),
        }
    }

    /// 📐️ Pile-type specific partial factors (γ_b, γ_s) and ground-test ξ3, ξ4: the sets R1 (bored 1.25/1.0, CFA 1.2/1.0, driven 1.0/1.0) and R4 (bored 1.6/1.3, CFA 1.45/1.3, driven 1.3/1.3) distinguish the pile type (EN 1997-1 Tables A.6-A.8, unverified), all other sets use the approach value.
    pub fn pile_type_factors(pile_type: &str, n_profiles: u32, base: &AnnexParams, approach: DesignApproach) -> (f64, f64, f64, f64) {
        let (xi3, xi4) = pile_correlation_factors(n_profiles);
        let key = pile_type.trim().to_ascii_lowercase();
        let bored = matches!(key.as_str(), "bored" | "bohr" | "bohrpfahl");
        let cfa = matches!(key.as_str(), "cfa" | "sob" | "schnecke");
        match approach {
            DesignApproach::Da1Geo => if bored { (1.6, 1.3, xi3, xi4) } else if cfa { (1.45, 1.3, xi3, xi4) } else { (1.3, 1.3, xi3, xi4) },
            DesignApproach::Da1Str => if bored { (1.25, 1.0, xi3, xi4) } else if cfa { (1.2, 1.0, xi3, xi4) } else { (1.0, 1.0, xi3, xi4) },
            _ => (base.gamma_b, base.gamma_s, xi3, xi4),
        }
    }

    pub fn pile_design_resistance_typed(r_b_k: f64, r_s_k: f64, gamma_b: f64, gamma_s: f64) -> f64 {
        r_b_k / gamma_b + r_s_k / gamma_s
    }

    pub fn is_cohesive_layer(layer: &SoilLayer) -> bool {
        let t = layer.soil_type.as_str().to_ascii_lowercase();
        layer.cohesion_undrained > 1.0
            || t.contains("clay")
            || t.contains("ton")
            || t.contains("cohesive")
    }

    /// 📐️ Rankine active earth-pressure coefficient K_a = tan²(45°−φ′/2).
    pub fn ka_rankine(phi_deg: f64) -> f64 { ep::ka_rankine(phi_deg, 0.0) }

    /// 📐️ Jáky at-rest coefficient K₀ = (1−sin φ′)·√OCR (Mayne-Kulhawy form).
    pub fn k0_jaky(phi_deg: f64, ocr: f64) -> f64 { ep::k0(phi_deg, ocr, 0.0) }

    /// 📐️ Select earth-pressure coefficient by DIN 4085 wall-movement regime.
    pub fn earth_pressure_coefficient(phi_deg: f64, mode: &str, ocr: f64, delta_deg: f64) -> (f64, &'static str) {
        let key = mode.trim().to_ascii_lowercase().replace(['-', '_', ' '], "");
        match key.as_str() {
            "atrest" | "k0" | "ruhe" => (k0_jaky(phi_deg, ocr), "K0"),
            "increasedactive" | "erhohteaktiv" | "erhöhtaktiv" | "increased" => {
                (0.5 * (ep::ka_coulomb(phi_deg, delta_deg, 0.0, 0.0) + k0_jaky(phi_deg, ocr)), "Ka*")
            }
            _ => (ep::ka_coulomb(phi_deg, delta_deg, 0.0, 0.0), "Ka-Coulomb"),
        }
    }

    fn layer_props_at(layers: &[SoilLayer], depth: f64) -> (f64, f64, f64, f64) {
        let layer = layers.iter().find(|l| depth >= l.depth_top && depth <= l.depth_bottom).or_else(|| layers.first());
        match layer {
            Some(l) => (l.phi_prime_deg, l.cohesion_effective, l.gamma, l.gamma_prime),
            None => (30.0, 0.0, 18_000.0, 10_000.0),
        }
    }

    /// 📐️ Simplified Bishop factor of safety by slice equilibrium over a circle search (EN 1997-1 §11); the partial factors are those of the overall-stability set (`Geo3`); NaN (indeterminate, never a pass) when the search is cancelled or no admissible circle exists.
    /// 📐️ Returns None when `governing_layer_id` is empty or not found among layers.
    pub fn bishop_fos_slices(
        layers: &[SoilLayer], governing_layer_id: &str, height: f64, angle_deg: f64, length: f64, gwl: f64, gamma_phi: f64, gamma_c: f64,
    ) -> Option<f64> {
        bishop_fos_slices_with(layers, governing_layer_id, height, angle_deg, length, gwl, gamma_phi, gamma_c, &mut |_| true)
    }

    /// 📐️ [`bishop_fos_slices`] for a worker: `step(fraction)` is called once per searched circle and returns `false` to cancel (the partial factor is then meaningless and the caller discards it).
    #[allow(clippy::too_many_arguments)]
    pub fn bishop_fos_slices_with(
        layers: &[SoilLayer], governing_layer_id: &str, height: f64, angle_deg: f64, length: f64, gwl: f64, gamma_phi: f64, gamma_c: f64, step: &mut dyn FnMut(f32) -> bool,
    ) -> Option<f64> {
        if governing_layer_id.trim().is_empty() || !layers.iter().any(|l| l.id == governing_layer_id) {
            return None;
        }
        Some(bishop_fos_slices_inner(layers, height, angle_deg, length, gwl, gamma_phi, gamma_c, step))
    }

    fn bishop_fos_slices_inner(
        layers: &[SoilLayer], height: f64, angle_deg: f64, length: f64, gwl: f64, gamma_phi: f64, gamma_c: f64, step: &mut dyn FnMut(f32) -> bool,
    ) -> f64 {
        let beta = angle_deg.to_radians().clamp(0.05, 1.45);
        let run = height / beta.tan();
        let horiz = length.max(run);
        let mut best = f64::INFINITY;
        let mut searched = 0;
        for &xc_f in &[0.0, 0.15, 0.3, 0.45, 0.6, 0.75, 0.9, 1.05] {
            for &yc_f in &[1.0, 1.4, 1.8, 2.4, 3.2] {
                for &depth_f in &[0.0, 0.15, 0.3, 0.6, 1.0] {
                    searched += 1;
                    if !step(searched as f32 / 200.0) {
                        return f64::NAN;
                    }
                    let (xc, yc) = (xc_f * horiz, yc_f * height);
                    let fos = bishop_one_circle(layers, height, beta, run, horiz, xc, yc, yc + depth_f * height, gwl, gamma_phi, gamma_c, 60);
                    if fos.is_finite() && fos > 0.0 {
                        best = best.min(fos);
                    }
                }
            }
        }
        if best.is_finite() { best } else { f64::NAN }
    }

    fn bishop_one_circle(
        layers: &[SoilLayer], height: f64, beta: f64, run: f64, margin: f64, xc: f64, yc: f64, r: f64, gwl: f64,
        gamma_phi: f64, gamma_c: f64, n_slices: usize,
    ) -> f64 {
        bishop_circle_detail(layers, height, beta, run, margin, xc, yc, r, gwl, gamma_phi, gamma_c, n_slices).map_or(f64::NAN, |d| d.0)
    }

    /// 🍰️ Bishop factor of one circle with the local mobilisation F·d_i/r_i of every slice (`(s, μ)`, `s` in `[0, 1]` across the slip width); `None` when no sliding mass develops.
    fn bishop_circle_detail(
        layers: &[SoilLayer], height: f64, beta: f64, run: f64, margin: f64, xc: f64, yc: f64, r: f64, gwl: f64,
        gamma_phi: f64, gamma_c: f64, n_slices: usize,
    ) -> Option<(f64, Vec<(f64, f64)>)> {
        let ground = |x: f64| if x <= 0.0 { height } else if x >= run { 0.0 } else { height - x * beta.tan() };
        let base = |x: f64| { let under = r * r - (x - xc).powi(2); if under < 0.0 { None } else { Some(yc - under.sqrt()) } };
        let (mut xa, mut xb) = (f64::NAN, f64::NAN);
        for i in 0..=800 {
            let x = xc - r + 2.0 * r * i as f64 / 800.0;
            if let Some(y) = base(x) {
                if y < ground(x) {
                    if xa.is_nan() { xa = x; }
                    xb = x;
                }
            }
        }
        if xa.is_nan() || xb - xa < 1e-6 || xa < -margin - 1e-9 || xb > run + margin + 1e-9 {
            return None;
        }
        let dx = (xb - xa) / n_slices as f64;
        let mut slices = Vec::with_capacity(n_slices);
        let mut drive = 0.0;
        for i in 0..n_slices {
            let x = xa + (i as f64 + 0.5) * dx;
            let (Some(yb), yg) = (base(x), ground(x)) else { return None };
            let h_slice = (yg - yb).max(0.0);
            if h_slice < 1e-9 {
                continue;
            }
            let mid_depth = (height - 0.5 * (yg + yb)).max(0.0);
            let (phi, c, gamma, gamma_p) = layer_props_at(layers, mid_depth);
            let y_w = yg - gwl;
            let wet = (y_w - yb).max(0.0).min(h_slice);
            let w = (gamma * (h_slice - wet) + (gamma_p + 9_810.0) * wet) * dx;
            let u = 9_810.0 * (y_w - yb).max(0.0) * dx;
            let alpha = ((xc - x) / r).asin();
            drive += w * alpha.sin();
            slices.push((w, u, alpha, (phi.to_radians().tan() / gamma_phi), c / gamma_c, dx, (x - xa) / (xb - xa)));
        }
        if drive <= 1e-9 {
            return None;
        }
        let mut f = 1.5_f64;
        for _ in 0..200 {
            let mut resist = 0.0;
            for &(w, u, alpha, tan_phi, c, b, _) in &slices {
                let m_alpha = alpha.cos() + alpha.sin() * tan_phi / f;
                if m_alpha <= 0.2 {
                    return None;
                }
                resist += (c * b + (w - u).max(0.0) * tan_phi) / m_alpha;
            }
            let next = resist / drive;
            if !next.is_finite() || next <= 0.0 {
                return None;
            }
            if (next - f).abs() < 1e-9 {
                return Some((next, mobilisation(&slices, next)));
            }
            f = next;
        }
        Some((f, mobilisation(&slices, f)))
    }

    fn mobilisation(slices: &[(f64, f64, f64, f64, f64, f64, f64)], f: f64) -> Vec<(f64, f64)> {
        slices.iter().map(|&(w, u, alpha, tan_phi, c, b, s)| {
            let resist = (c * b + (w - u).max(0.0) * tan_phi) / (alpha.cos() + alpha.sin() * tan_phi / f);
            (s, if resist > 1e-9 { (f * w * alpha.sin() / resist).max(0.0) } else { 0.0 })
        }).collect()
    }

    /// 🍰️ Local slice mobilisation `(s, μ)` of the critical Bishop circle of a slope, found with the same circle grid as [`bishop_fos_slices`]; `None` when no sliding mass develops.
    pub fn bishop_critical_slices(
        layers: &[SoilLayer], governing_layer_id: &str, height: f64, angle_deg: f64, length: f64, gwl: f64, gamma_phi: f64, gamma_c: f64,
    ) -> Option<Vec<(f64, f64)>> {
        if governing_layer_id.trim().is_empty() || !layers.iter().any(|l| l.id == governing_layer_id) {
            return None;
        }
        let beta = angle_deg.to_radians().clamp(0.05, 1.45);
        let run = height / beta.tan();
        let horiz = length.max(run);
        let mut best: Option<(f64, Vec<(f64, f64)>)> = None;
        for &xc_f in &[0.0, 0.15, 0.3, 0.45, 0.6, 0.75, 0.9, 1.05] {
            for &yc_f in &[1.0, 1.4, 1.8, 2.4, 3.2] {
                for &depth_f in &[0.0, 0.15, 0.3, 0.6, 1.0] {
                    let (xc, yc) = (xc_f * horiz, yc_f * height);
                    if let Some((fos, slices)) = bishop_circle_detail(layers, height, beta, run, horiz, xc, yc, yc + depth_f * height, gwl, gamma_phi, gamma_c, 60) {
                        if fos.is_finite() && fos > 0.0 && best.as_ref().map_or(true, |(b, _)| fos < *b) {
                            best = Some((fos, slices));
                        }
                    }
                }
            }
        }
        best.map(|(_, slices)| slices)
    }

    /// 🎯️ Bishop factor and slice mobilisation of ONE user-chosen circle (centre abscissa from the crest, centre elevation above the toe, radius); `None` for an unknown governing layer, `NaN` factor when the circle develops no sliding mass.
    pub fn bishop_fos_circle(
        layers: &[SoilLayer], governing_layer_id: &str, height: f64, angle_deg: f64, length: f64, gwl: f64, gamma_phi: f64, gamma_c: f64, centre_x: f64, centre_z: f64, radius: f64,
    ) -> Option<(f64, Vec<(f64, f64)>)> {
        if governing_layer_id.trim().is_empty() || !layers.iter().any(|l| l.id == governing_layer_id) {
            return None;
        }
        let beta = angle_deg.to_radians().clamp(0.05, 1.45);
        let run = height / beta.tan();
        let horiz = length.max(run);
        Some(bishop_circle_detail(layers, height, beta, run, horiz, centre_x, centre_z, radius, gwl, gamma_phi, gamma_c, 60).filter(|d| d.0.is_finite() && d.0 > 0.0).unwrap_or((f64::NAN, Vec::new())))
    }

    /// 📐️ Registry key of the soil class in the GK1 presumptive bearing table.
    pub fn presumptive_soil_key(soil_type: &str) -> &'static str {
        match soil_type.to_ascii_lowercase().as_str() {
            "gravel" | "kies" => "gravel",
            "sand" | "sandig" => "sand",
            "silt" | "schluff" => "silt",
            "clay" | "ton" => "clay",
            _ => "other",
        }
    }

    /// 📐️ DIN 1054 Table A.6.10-style presumptive design bearing pressure [Pa] for GK1 from the unverified registry rows `ndp.en1997.gk1.presumptive.*`.
    pub fn presumptive_bearing_pa(soil_type: &str, embedment: f64) -> f64 {
        let base = semio_s_plugin_norm_compliance::ndp_cell_common("ndp.en1997.gk1.presumptive", &[presumptive_soil_key(soil_type)]);
        let gain = semio_s_plugin_norm_compliance::ndp_cell_common("ndp.en1997.gk1.presumptive", &["embedment-gain"]);
        let cap = semio_s_plugin_norm_compliance::ndp_cell_common("ndp.en1997.gk1.presumptive", &["embedment-cap"]);
        base * (1.0 + gain * embedment.min(cap))
    }
}

pub mod part_2 {
    use crate::*;

    /// 📐️ φ′ [°] from CPT tip resistance after Kulhawy and Mayne, φ′ = 17.6° + 11°·log10((q_c/p_a)/√(σ′v0/p_a)), p_a = 100 kPa.
    pub fn phi_from_cpt_deg(q_c_pa: f64, sigma_v0_eff_pa: f64) -> f64 {
        let pa = 100_000.0;
        17.6 + 11.0 * ((q_c_pa / pa).max(1e-9) / (sigma_v0_eff_pa.max(1.0) / pa).sqrt()).log10()
    }

    /// 📐️ φ′ [°] from the SPT blow count: (N1)60 = N·√(100 kPa/σ′v0) with C_N ≤ 1.7 and φ′ = √(20·(N1)60) + 20° (Hatanaka-Uchida).
    pub fn phi_from_spt_deg(n_spt: f64, sigma_v0_eff_pa: f64) -> f64 {
        (20.0 * n1_60(n_spt, sigma_v0_eff_pa)).sqrt() + 20.0
    }

    /// 📐️ Overburden-normalised blow count (N1)60 = N60·C_N with C_N = √(100 kPa/σ′v0) ≤ 1.7 (GI-06); the energy ratio and the rod, borehole and sampler corrections are not recorded on an investigation point, so N60 = N.
    pub fn n1_60(n_spt: f64, sigma_v0_eff_pa: f64) -> f64 {
        n_spt * (100_000.0 / sigma_v0_eff_pa.max(1.0)).sqrt().min(1.7)
    }

    /// 📐️ Minimum investigation depth z_a ≥ max(6 m, 2·B) for spread foundations (EN 1997-2 B.3).
    pub fn min_investigation_depth_m(b: f64) -> f64 { (6.0f64).max(2.0 * b) }

    /// 📐️ Minimum investigation depth by geotechnical category: the depth rule is category independent, GK 3 additionally tightens the point spacing.
    pub fn min_investigation_depth_for_gk(b: f64, _gk: u8) -> f64 { min_investigation_depth_m(b) }

    /// 📐️ Investigation-only φ′ from CPT/SPT correlations (EN 1997-2); ignores stated φ′.
    pub fn investigation_phi_deg(layer: &SoilLayer, overburden_pa: f64) -> Option<(f64, &'static str)> {
        let sigma = overburden_pa.max(50_000.0);
        let mut best: Option<(f64, &'static str)> = None;
        if layer.cpt_qc > 0.0 {
            best = Some((phi_from_cpt_deg(layer.cpt_qc, sigma), "cpt"));
        }
        if layer.spt_n > 0.0 {
            let derived = phi_from_spt_deg(layer.spt_n, sigma);
            best = match best {
                Some((phi, _)) if derived + 1e-9 < phi => Some((derived, "spt")),
                Some(other) => Some(other),
                None => Some((derived, "spt")),
            };
        }
        best
    }

    /// 📐️ Cautious characteristic φ′ = min(stated, investigation) per EN 1997-1 §2.4.5.2.
    pub fn characteristic_phi_deg(layer: &SoilLayer, overburden_pa: f64) -> (f64, &'static str) {
        let mut phi = layer.phi_prime_deg;
        let mut src = "stated";
        if let Some((derived, dsrc)) = investigation_phi_deg(layer, overburden_pa) {
            if derived + 1e-9 < phi {
                phi = derived;
                src = dsrc;
            }
        }
        (phi, src)
    }
}
