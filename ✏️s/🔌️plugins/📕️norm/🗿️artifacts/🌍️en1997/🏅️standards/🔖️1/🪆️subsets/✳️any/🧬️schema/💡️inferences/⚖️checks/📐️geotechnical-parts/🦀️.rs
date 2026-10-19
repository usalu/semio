//! 📐️ EN 1997 geotechnical parts: the clause modules of EN 1997-1 and EN 1997-2 with their shared subject helpers.

use super::*;

pub(super) fn loc(en: &str, de: &str) -> LocalizedCopy { LocalizedCopy::new(en, de) }

pub(super) fn subject_entity(id: &str, path: &str, en: &str, de: &str) -> SubjectRef {
    SubjectRef::new(id, path, loc(en, de))
}

pub mod part_1 {
    use crate::*;

    /// 📐️ Annex D bearing factor N_q = e^(π·tanφ)·tan²(45+φ/2).
    pub fn bearing_factor_n_q(phi_deg: f64) -> f64 {
        let phi = phi_deg.to_radians();
        (std::f64::consts::PI * phi.tan()).exp() * (std::f64::consts::FRAC_PI_4 + phi / 2.0).tan().powi(2)
    }

    /// 📐️ Annex D bearing factor N_c = (N_q − 1)·cot φ.
    pub fn bearing_factor_n_c(phi_deg: f64) -> f64 {
        if phi_deg.abs() < 1e-9 { return 5.14; }
        let phi = phi_deg.to_radians();
        (bearing_factor_n_q(phi_deg) - 1.0) / phi.tan()
    }

    /// 📐️ Annex D bearing factor N_γ = 2·(N_q − 1)·tan φ.
    pub fn bearing_factor_n_gamma(phi_deg: f64) -> f64 {
        2.0 * (bearing_factor_n_q(phi_deg) - 1.0) * phi_deg.to_radians().tan()
    }

    /// 📐️ Shape/depth/inclination factors and effective widths (Annex D).
    pub fn effective_width(b: f64, eccentricity: f64) -> f64 {
        (b - 2.0 * eccentricity.abs()).max(1e-6)
    }

    pub fn shape_factors(phi_deg: f64, b_prime: f64, l_prime: f64) -> (f64, f64, f64) {
        let n_q = bearing_factor_n_q(phi_deg);
        let n_c = bearing_factor_n_c(phi_deg);
        let ratio = (b_prime / l_prime.max(b_prime)).clamp(0.0, 1.0);
        let s_q = 1.0 + ratio * phi_deg.to_radians().sin();
        let s_gamma = 1.0 - 0.3 * ratio;
        let s_c = if n_c.abs() < 1e-9 { 1.0 } else { (s_q * n_q - 1.0) / (n_c.max(1e-9)) };
        (s_c, s_q, s_gamma.max(0.7))
    }

    pub fn depth_factors(phi_deg: f64, embedment: f64, b_prime: f64) -> (f64, f64, f64) {
        let phi = phi_deg.to_radians();
        let d_b = embedment / b_prime.max(1e-6);
        let d_q = 1.0 + 2.0 * phi.tan() * (1.0 - phi.sin()).powi(2) * d_b.atan();
        let d_c = if phi_deg.abs() < 1e-9 { 1.0 + 0.4 * d_b.atan() } else { d_q - (1.0 - d_q) / (bearing_factor_n_c(phi_deg) * phi.tan()).max(1e-9) };
        let d_gamma = 1.0;
        (d_c, d_q, d_gamma)
    }

    pub fn inclination_factors(phi_deg: f64, h: f64, v: f64, c: f64, b_prime: f64, l_prime: f64) -> (f64, f64, f64) {
        let a = b_prime * l_prime;
        let m = (2.0 + (b_prime / l_prime.max(b_prime))) / (1.0 + (b_prime / l_prime.max(b_prime)));
        let n_c = bearing_factor_n_c(phi_deg);
        let denom = (v + a * c / n_c.max(1e-6)).max(1e-6);
        let ratio = (1.0 - h.abs() / denom).clamp(0.0, 1.0);
        let i_q = ratio.powf(m);
        let i_c = if phi_deg.abs() < 1e-9 { 1.0 - m * h.abs() / (a * c * n_c + 1e-6).max(1e-6) } else {
            i_q - (1.0 - i_q) / (n_c * phi_deg.to_radians().tan()).max(1e-9)
        };
        let i_gamma = ratio.powf(m + 1.0);
        (i_c.max(0.0), i_q.max(0.0), i_gamma.max(0.0))
    }

    /// 📐️ Annex D base-inclination factors b_c, b_q, b_γ for footing base tilt α.
    pub fn base_inclination_factors(phi_deg: f64, alpha_deg: f64) -> (f64, f64, f64) {
        let alpha = alpha_deg.to_radians().abs();
        let phi = phi_deg.to_radians();
        if alpha < 1e-12 {
            return (1.0, 1.0, 1.0);
        }
        let b_q = (1.0 - alpha.tan() * phi.tan()).max(0.0).powi(2);
        let b_gamma = b_q;
        let n_c = bearing_factor_n_c(phi_deg);
        let b_c = if phi_deg.abs() < 1e-9 {
            1.0 - 2.0 * alpha / std::f64::consts::PI
        } else {
            b_q - (1.0 - b_q) / (n_c * phi.tan()).max(1e-9)
        };
        (b_c.max(0.0), b_q.max(0.0), b_gamma.max(0.0))
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

    /// 📐️ Ultimate bearing pressure q_u [Pa] per EN 1997-1 Annex D (incl. base inclination).
    pub fn ultimate_bearing_pa(
        phi_deg: f64, c_pa: f64, gamma_overburden_pa_m: f64, gamma_below_pa_m: f64,
        b: f64, l: f64, embedment: f64, eccentricity_b: f64, eccentricity_l: f64, h: f64, v: f64,
        base_inclination_deg: f64,
    ) -> f64 {
        let b_p = effective_width(b, eccentricity_b);
        let l_p = effective_width(l, eccentricity_l);
        let (s_c, s_q, s_g) = shape_factors(phi_deg, b_p, l_p);
        let (d_c, d_q, d_g) = depth_factors(phi_deg, embedment, b_p);
        let (i_c, i_q, i_g) = inclination_factors(phi_deg, h, v, c_pa, b_p, l_p);
        let (b_c, b_q, b_g) = base_inclination_factors(phi_deg, base_inclination_deg);
        let n_c = bearing_factor_n_c(phi_deg);
        let n_q = bearing_factor_n_q(phi_deg);
        let n_g = bearing_factor_n_gamma(phi_deg);
        let q = gamma_overburden_pa_m * embedment;
        c_pa * n_c * s_c * d_c * i_c * b_c
            + q * n_q * s_q * d_q * i_q * b_q
            + 0.5 * gamma_below_pa_m * b_p * n_g * s_g * d_g * i_g * b_g
    }

    pub fn design_bearing_resistance_n(
        phi_deg: f64, c_pa: f64, gamma_pa_m: f64, b: f64, l: f64, embedment: f64,
        e_b: f64, e_l: f64, h: f64, v: f64, approach: DesignApproach, annex: AnnexChoice,
    ) -> f64 {
        design_bearing_resistance_with_params(phi_deg, c_pa, gamma_pa_m, gamma_pa_m, b, l, embedment, e_b, e_l, h, v, 0.0, &approach.annex_params(annex))
    }

    pub fn design_bearing_resistance_with_params(
        phi_deg: f64, c_pa: f64, gamma_over_pa_m: f64, gamma_below_pa_m: f64, b: f64, l: f64, embedment: f64,
        e_b: f64, e_l: f64, h: f64, v: f64, base_inclination_deg: f64, p: &AnnexParams,
    ) -> f64 {
        let phi_d = (phi_deg.to_radians() / p.gamma_phi).atan().to_degrees();
        let c_d = c_pa / p.gamma_c;
        let gamma_o = gamma_over_pa_m / p.gamma_gamma;
        let gamma_b = gamma_below_pa_m / p.gamma_gamma;
        let b_p = effective_width(b, e_b);
        let l_p = effective_width(l, e_l);
        let q_u = ultimate_bearing_pa(phi_d, c_d, gamma_o, gamma_b, b, l, embedment, e_b, e_l, h, v, base_inclination_deg);
        q_u * b_p * l_p / p.gamma_r_v
    }

    /// 📐️ Passive earth-pressure force E_p,k [N] on the front face (Coulomb K_p).
    pub fn passive_earth_force_n(phi_deg: f64, gamma_pa_m: f64, embedment: f64, length: f64, wall_friction_deg: f64) -> f64 {
        if embedment <= 0.0 || length <= 0.0 {
            return 0.0;
        }
        let kp = kp_coulomb(phi_deg, wall_friction_deg);
        0.5 * gamma_pa_m * embedment * embedment * kp * length
    }

    pub fn sliding_resistance_n(phi_deg: f64, c_pa: f64, v_n: f64, a: f64, approach: DesignApproach, annex: AnnexChoice) -> f64 {
        sliding_resistance_with_params(phi_deg, c_pa, v_n, a, &approach.annex_params(annex), 0.0, 0.0, 18_000.0, false)
    }

    pub fn sliding_resistance_with_params(
        phi_deg: f64, c_pa: f64, v_n: f64, a: f64, p: &AnnexParams,
        embedment: f64, length: f64, gamma_pa_m: f64, permanent_embedment: bool,
    ) -> f64 {
        let phi_d = (phi_deg.to_radians() / p.gamma_phi).atan();
        let c_d = c_pa / p.gamma_c;
        let base = c_d * a + v_n * phi_d.tan();
        let passive = if permanent_embedment && embedment > 0.0 {
            0.5 * passive_earth_force_n(phi_deg, gamma_pa_m, embedment, length, 0.0)
        } else {
            0.0
        };
        (base + passive) / p.gamma_r_h
    }

    /// 📐️ Vertical stress increase Δσ [Pa] at depth z below foundation base (2:1).
    pub fn stress_increase_2to1_pa(q_pa: f64, width: f64, length: f64, z: f64) -> f64 {
        let b = width.max(1e-6);
        let l = length.max(b);
        let zz = z.max(0.0);
        q_pa * (b * l) / ((b + zz) * (l + zz))
    }

    /// 📐️ Layer-wise oedometric settlement [m] (EN 1997-1 §6.6); returns (s, governing layer id).
    pub fn settlement_oedometric_m(layers: &[SoilLayer], width: f64, length: f64, embedment: f64, q_sls_pa: f64) -> (f64, String) {
        let (s, id, _) = settlement_oedometric_with_gwl(layers, width, length, embedment, q_sls_pa, f64::INFINITY);
        (s, id)
    }

    /// 📐️ Oedometric settlement with GWL; returns `(s, governing_layer_id, governing_layer_contribution)`.
    pub fn settlement_oedometric_with_gwl(
        layers: &[SoilLayer], width: f64, length: f64, embedment: f64, q_sls_pa: f64, gwl: f64,
    ) -> (f64, String, f64) {
        let mut s = 0.0;
        let mut best_id = String::new();
        let mut best_contrib = 0.0;
        let gamma_w = 9_810.0;
        for layer in layers {
            let z_top = (layer.depth_top - embedment).max(0.0);
            let z_bot = (layer.depth_bottom - embedment).max(0.0);
            if z_bot <= z_top + 1e-12 {
                continue;
            }
            let dz = z_bot - z_top;
            let z_mid = 0.5 * (z_top + z_bot);
            let mut dsig = stress_increase_2to1_pa(q_sls_pa, width, length, z_mid);
            let depth_abs = embedment + z_mid;
            if gwl < depth_abs {
                let gamma_eff = layer.gamma_prime.min((layer.gamma - gamma_w).max(0.0)).max(1.0);
                dsig *= (gamma_eff / layer.gamma.max(1.0)).clamp(0.4, 1.0);
            }
            let e = layer.oedometric_modulus.max(1.0);
            let contrib = dsig / e * dz;
            s += contrib;
            if contrib >= best_contrib {
                best_contrib = contrib;
                best_id = layer.id.clone();
            }
        }
        if best_id.is_empty() {
            if let Some(first) = layers.first() {
                best_id = first.id.clone();
                let e = first.oedometric_modulus.max(1.0);
                s = q_sls_pa / e * width.max(0.1) * 0.5;
                best_contrib = s;
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

    pub fn pile_correlation_factors(n_profiles: u32) -> (f64, f64) {
        match n_profiles {
            0 | 1 => (1.40, 1.40),
            2 => (1.35, 1.27),
            3 | 4 => {
                let t = (n_profiles - 2) as f64 / 3.0;
                (1.35 + t * (1.30 - 1.35), 1.27 + t * (1.15 - 1.27))
            }
            _ => (1.30, 1.15),
        }
    }

    pub fn pile_characteristic_resistance_n(mean: f64, min: f64, n: u32) -> f64 {
        let (xi3, xi4) = pile_correlation_factors(n);
        (mean / xi3).min(min / xi4)
    }

    pub fn pile_design_resistance_n(r_b_k: f64, r_s_k: f64, approach: DesignApproach, annex: AnnexChoice) -> f64 {
        pile_design_resistance_with_params(r_b_k, r_s_k, &approach.annex_params(annex))
    }

    pub fn pile_design_resistance_with_params(r_b_k: f64, r_s_k: f64, p: &AnnexParams) -> f64 {
        r_b_k / p.gamma_b + r_s_k / p.gamma_s
    }

    /// 📐️ Undrained Annex D.3 bearing pressure q_u = (π+2)·c_u·s_c·i_c + q [Pa].
    pub fn undrained_bearing_pa(c_u: f64, gamma: f64, b: f64, l: f64, embedment: f64, e_b: f64, h: f64, _v: f64) -> f64 {
        let b_p = effective_width(b, e_b);
        let l_p = effective_width(l, 0.0);
        let ratio = (b_p / l_p.max(b_p)).clamp(0.0, 1.0);
        let s_c = 1.0 + 0.2 * ratio;
        let a = b_p * l_p;
        let i_c = (1.0 - h.abs() / (a * c_u * (std::f64::consts::PI + 2.0) + 1e-6).max(1e-6)).clamp(0.0, 1.0);
        let q = gamma * embedment;
        (std::f64::consts::PI + 2.0) * c_u * s_c * i_c + q
    }

    pub fn undrained_bearing_resistance_n(c_u: f64, gamma: f64, b: f64, l: f64, embedment: f64, e_b: f64, h: f64, v: f64, p: &AnnexParams) -> f64 {
        let b_p = effective_width(b, e_b);
        let l_p = effective_width(l, 0.0);
        let q_u = undrained_bearing_pa(c_u / p.gamma_c.max(1.0), gamma / p.gamma_gamma, b, l, embedment, e_b, h, v);
        q_u * b_p * l_p / p.gamma_r_v
    }

    /// 📐️ Short-term undrained sliding R_d = A·c_u / γ_R,h [N].
    pub fn undrained_sliding_resistance_n(c_u: f64, a: f64, p: &AnnexParams) -> f64 {
        (c_u / p.gamma_c.max(1.0)) * a / p.gamma_r_h
    }

    /// 📐️ Immediate elastic settlement (EN 1997-1 Annex F.2 / DIN 4019): s = p·B·(1−ν²)·I / E.
    pub fn elastic_settlement_m(q_pa: f64, width: f64, length: f64, nu: f64, e_pa: f64, influence: f64) -> f64 {
        let e = e_pa.max(1.0);
        let b = width.max(0.1);
        let l = length.max(b);
        let i_f = influence.clamp(0.5, 1.5);
        let e_young = if nu > 0.0 && nu < 0.49 {
            e * (1.0 + nu) * (1.0 - 2.0 * nu) / (1.0 - nu).max(0.05)
        } else {
            e
        };
        q_pa * b * (1.0 - nu * nu) * i_f / e_young.max(1.0) * (b / l).sqrt().clamp(0.7, 1.0)
    }

    /// 📐️ Layered immediate settlement — each layer contributes with its own ν and E_oed.
    pub fn elastic_settlement_layered(layers: &[SoilLayer], width: f64, length: f64, embedment: f64, q_pa: f64) -> (f64, String) {
        let mut s = 0.0;
        let mut best_id = String::new();
        let mut best = 0.0;
        let b = width.max(0.1);
        let z_limit = layers.iter().map(|l| (l.depth_bottom - embedment).max(0.0)).fold(2.0 * b, f64::max);
        for layer in layers {
            let z_top = (layer.depth_top - embedment).max(0.0);
            if z_top > z_limit { continue; }
            let z_bot = (layer.depth_bottom - embedment).max(z_top).min(z_limit);
            let dz = (z_bot - z_top).max(0.0);
            if dz < 1e-9 { continue; }
            let z_mid = z_top + 0.5 * dz;
            let i_f = (1.0 / (1.0 + z_mid / b)).clamp(0.2, 1.0);
            let stress = q_pa * (b / (b + z_mid)).powi(2).clamp(0.05, 1.0);
            let ds = elastic_settlement_m(stress, width, length, layer.poisson_ratio, layer.oedometric_modulus, i_f) * (dz / (2.0 * b)).clamp(0.02, 1.0);
            if ds >= best {
                best = ds;
                best_id = layer.id.clone();
            }
            s += ds;
        }
        if best_id.is_empty() {
            if let Some(l) = layers.first() { best_id = l.id.clone(); }
        }
        (s, best_id)
    }

    /// 📐️ DIN 1054 / EA-Pfähle pile-type factors (γ_b, γ_s, ξ3, ξ4) for compression.
    pub fn pile_type_factors(pile_type: &str, n_profiles: u32, base: &AnnexParams) -> (f64, f64, f64, f64) {
        let (xi3, xi4) = pile_correlation_factors(n_profiles);
        let key = pile_type.trim().to_ascii_lowercase();
        match key.as_str() {
            "driven" | "ramm" | "rammpfahl" => {
                let (xi3, xi4) = match n_profiles {
                    0 | 1 => (1.35, 1.35),
                    2 => (1.30, 1.25),
                    _ => (1.25, 1.15),
                };
                (base.gamma_b.max(1.10), base.gamma_s.max(1.15), xi3, xi4)
            }
            "cfa" | "sob" | "schnecke" => (base.gamma_b.max(1.15), base.gamma_s.max(1.15), xi3.max(1.40), xi4.max(1.40)),
            _ => (base.gamma_b.max(1.10), base.gamma_s.max(1.10), xi3, xi4),
        }
    }

    pub fn pile_design_resistance_typed(r_b_k: f64, r_s_k: f64, gamma_b: f64, gamma_s: f64) -> f64 {
        r_b_k / gamma_b + r_s_k / gamma_s
    }

    pub fn is_cohesive_layer(layer: &SoilLayer) -> bool {
        let t = layer.soil_type.to_ascii_lowercase();
        layer.cohesion_undrained > 1.0
            || t.contains("clay")
            || t.contains("ton")
            || t.contains("cohesive")
    }

    /// 📐️ Rankine active earth-pressure coefficient K_a = tan²(45°−φ′/2).
    pub fn ka_rankine(phi_deg: f64) -> f64 {
        let alpha = std::f64::consts::FRAC_PI_4 - phi_deg.to_radians() / 2.0;
        alpha.tan().powi(2).max(0.05)
    }

    /// 📐️ Rankine passive K_p = tan²(45°+φ′/2).
    pub fn kp_rankine(phi_deg: f64) -> f64 {
        let alpha = std::f64::consts::FRAC_PI_4 + phi_deg.to_radians() / 2.0;
        alpha.tan().powi(2).min(20.0)
    }

    /// 📐️ Jáky at-rest coefficient K₀ = (1−sin φ′)·OCR^sin φ′.
    pub fn k0_jaky(phi_deg: f64, ocr: f64) -> f64 {
        let phi = phi_deg.to_radians();
        let k0_nc = (1.0 - phi.sin()).max(0.15);
        k0_nc * ocr.max(1.0).powf(phi.sin())
    }

    /// 📐️ Select earth-pressure coefficient by DIN 4085 wall-movement regime.
    pub fn earth_pressure_coefficient(phi_deg: f64, mode: &str, ocr: f64, delta_deg: f64) -> (f64, &'static str) {
        let key = mode.trim().to_ascii_lowercase().replace(['-', '_', ' '], "");
        match key.as_str() {
            "atrest" | "k0" | "ruhe" => (k0_jaky(phi_deg, ocr), "K0"),
            "increasedactive" | "erhohteaktiv" | "erhöhtaktiv" | "increased" => {
                let ka = ka_rankine(phi_deg);
                let k0 = k0_jaky(phi_deg, ocr);
                (0.5 * (ka + k0), "Ka*")
            }
            _ => {
                if delta_deg.abs() > 1e-6 {
                    (ka_coulomb(phi_deg, delta_deg), "Ka-Coulomb")
                } else {
                    (ka_rankine(phi_deg), "Ka-Rankine")
                }
            }
        }
    }

    pub fn ka_coulomb(phi_deg: f64, delta_deg: f64) -> f64 {
        let phi = phi_deg.to_radians();
        let delta = delta_deg.to_radians();
        let num = (phi.cos()).powi(2);
        let den = ((phi.cos() + (phi.sin() * (phi - delta).sin().max(0.0) / (delta.cos()).max(1e-6)).sqrt()).max(1e-6)).powi(2);
        (num / den / phi.cos().max(1e-6)).max(0.1)
    }

    pub fn kp_coulomb(phi_deg: f64, delta_deg: f64) -> f64 {
        let ka = ka_coulomb(phi_deg, delta_deg);
        (1.0 / ka).min(20.0)
    }

    fn layer_props_at(layers: &[SoilLayer], depth: f64) -> (f64, f64, f64, f64) {
        let layer = layers.iter().find(|l| depth >= l.depth_top && depth <= l.depth_bottom).or_else(|| layers.first());
        match layer {
            Some(l) => (l.phi_prime_deg, l.cohesion_effective, l.gamma, l.gamma_prime),
            None => (30.0, 0.0, 18_000.0, 10_000.0),
        }
    }

    /// 📐️ Simplified Bishop FoS with slice equilibrium + circular slip search (EN 1997-1 §11).
    /// Overall stability partial factors are GEO-3 (EN 1997-1 §2.4.7.3.4 and DIN 1054 National Annex), independent of the project design approach.
    /// 📐️ Returns None when `governing_layer_id` is empty or not found among layers.
    pub fn bishop_fos_slices(
        layers: &[SoilLayer], governing_layer_id: &str, height: f64, angle_deg: f64, length: f64, gwl: f64, gamma_phi: f64, gamma_c: f64,
    ) -> Option<f64> {
        if governing_layer_id.trim().is_empty() || !layers.iter().any(|l| l.id == governing_layer_id) {
            return None;
        }
        Some(bishop_fos_slices_inner(layers, height, angle_deg, length, gwl, gamma_phi, gamma_c))
    }

    fn bishop_fos_slices_inner(
        layers: &[SoilLayer], height: f64, angle_deg: f64, length: f64, gwl: f64, gamma_phi: f64, gamma_c: f64,
    ) -> f64 {
        let beta = angle_deg.to_radians().clamp(0.05, 1.45);
        let horiz = if length > 0.5 {
            length.max(height / beta.tan().max(0.05))
        } else {
            height / beta.tan().max(0.05)
        };
        let n_slices = 16usize;
        let mut best = f64::INFINITY;
        let toe_x = 0.0;
        let crest_x = horiz;
        for &xc_frac in &[0.25, 0.4, 0.55, 0.7, 0.85] {
            for &yc_frac in &[0.8, 1.1, 1.4, 1.8, 2.2] {
                let xc = toe_x + xc_frac * (crest_x - toe_x);
                let yc = height * yc_frac;
                let r_toe = ((xc - toe_x).hypot(yc - 0.0)).max(height * 0.5);
                let r_crest = ((xc - crest_x).hypot(yc - height)).max(height * 0.5);
                for &r in &[r_toe, r_crest, 0.5 * (r_toe + r_crest), r_toe * 1.15, r_crest * 1.15] {
                    let fos = bishop_one_circle(layers, height, beta, horiz, xc, yc, r, gwl, gamma_phi, gamma_c, n_slices);
                    if fos.is_finite() && fos > 0.05 && fos < 50.0 {
                        best = best.min(fos);
                    }
                }
            }
        }
        if best.is_finite() {
            best
        } else {
            let (phi, c, gamma, _) = layer_props_at(layers, height * 0.5);
            bishop_fos(phi, c, gamma, height, angle_deg, gamma_phi, gamma_c)
        }
    }

    fn bishop_one_circle(
        layers: &[SoilLayer], height: f64, beta: f64, horiz: f64, xc: f64, yc: f64, r: f64, gwl: f64,
        gamma_phi: f64, gamma_c: f64, n_slices: usize,
    ) -> f64 {
        let gamma_w = 9_810.0;
        let dx = horiz / n_slices as f64;
        let mut f = 1.5_f64;
        for _ in 0..40 {
            let mut num = 0.0;
            let mut den = 0.0;
            let mut used = 0usize;
            for i in 0..n_slices {
                let x = (i as f64 + 0.5) * dx;
                let y_surface = (height - x * beta.tan()).max(0.0);
                if x > horiz {
                    continue;
                }
                let under = r * r - (x - xc).powi(2);
                if under <= 0.0 {
                    continue;
                }
                let y_slip = yc - under.sqrt();
                if y_slip >= y_surface - 1e-9 {
                    continue;
                }
                let h_slice = (y_surface - y_slip).max(0.0);
                if h_slice < 1e-6 {
                    continue;
                }
                let mid_depth = (height - 0.5 * (y_surface + y_slip)).max(0.0);
                let (phi, c, gamma, gamma_p) = layer_props_at(layers, mid_depth);
                let phi_d = (phi.to_radians().tan() / gamma_phi).atan();
                let c_d = c / gamma_c;
                let slip_depth_from_crest = height - y_slip;
                let submerged = gwl < slip_depth_from_crest;
                let gamma_eff = if submerged { gamma_p.max(1.0) } else { gamma };
                let b = dx;
                let w = gamma_eff * h_slice * b;
                let u = if submerged {
                    gamma_w * (gwl - (height - 0.5 * (y_surface + y_slip))).max(0.0)
                } else {
                    0.0
                };
                let alpha = ((x - xc) / r).asin().clamp(-1.2, 1.2);
                let m_alpha = (alpha.cos() + alpha.sin() * phi_d.tan() / f.max(0.1)).max(0.05);
                num += (c_d * b + (w - u * b).max(0.0) * phi_d.tan()) / m_alpha;
                den += w * alpha.sin();
                used += 1;
            }
            if used < n_slices / 3 || den.abs() < 1e-6 {
                return f64::NAN;
            }
            let f_new = num / den;
            if !f_new.is_finite() || f_new <= 0.0 {
                return f64::NAN;
            }
            if (f_new - f).abs() < 1e-4 {
                return f_new.clamp(0.05, 50.0);
            }
            f = 0.5 * (f + f_new);
        }
        f.clamp(0.05, 50.0)
    }

    /// 📐️ Legacy closed-form Bishop (fallback only).
    pub fn bishop_fos(phi_deg: f64, c_pa: f64, gamma_pa_m: f64, height: f64, angle_deg: f64, gamma_phi: f64, gamma_c: f64) -> f64 {
        let phi_d = (phi_deg.to_radians().tan() / gamma_phi).atan();
        let c_d = c_pa / gamma_c;
        let beta = angle_deg.to_radians();
        let weight = gamma_pa_m * height * height / (2.0 * beta.tan().max(0.2));
        let resisting = c_d * height / beta.sin().max(0.2) + weight * phi_d.tan() * beta.cos() / beta.sin().max(0.2);
        let driving = weight * beta.sin().max(0.05);
        resisting / driving.max(1.0)
    }

    /// 📐️ DIN 1054 Table A.6.10-style presumptive design bearing pressure [Pa] for GK1.
    pub fn presumptive_bearing_pa(soil_type: &str, embedment: f64) -> f64 {
        let base = match soil_type.to_ascii_lowercase().as_str() {
            "gravel" | "kies" => 400_000.0,
            "sand" | "sandig" => 200_000.0,
            "silt" | "schluff" => 150_000.0,
            "clay" | "ton" => 120_000.0,
            _ => 180_000.0,
        };
        base * (1.0 + 0.1 * embedment.min(3.0))
    }
}

pub mod part_2 {
    use crate::*;
    /// 📐️ φ′ from CPT tip resistance (EN 1997-2 / Robertson-type correlation, cautious).
    pub fn phi_from_cpt_deg(q_c_pa: f64, sigma_v0_pa: f64) -> f64 {
        let ratio = (q_c_pa / sigma_v0_pa.max(1.0)).max(1.0);
        (27.0 + 10.0 * (ratio.log10() - 0.5)).clamp(28.0, 42.0)
    }
    /// 📐️ φ′ from SPT blow count N (EN 1997-2 empirical correlation, cautious).
    pub fn phi_from_spt_deg(n_spt: f64) -> f64 {
        (27.1 + 0.3 * n_spt - 0.00054 * n_spt * n_spt).clamp(25.0, 42.0)
    }
    pub fn min_investigation_depth_m(b: f64) -> f64 { 3.0 * b }
    pub fn min_investigation_depth_for_gk(b: f64, gk: u8) -> f64 {
        match gk {
            1 => 2.0 * b,
            3 => 3.5 * b,
            _ => 3.0 * b,
        }
    }
    /// 📐️ Characteristic φ′ from investigation: min(stated, CPT/SPT-derived when data present).
    /// 📐️ Investigation-only φ′ from CPT/SPT correlations (EN 1997-2); ignores stated φ′.
    pub fn investigation_phi_deg(layer: &SoilLayer, overburden_pa: f64) -> Option<(f64, &'static str)> {
        let mut best: Option<(f64, &'static str)> = None;
        if layer.cpt_qc > 0.0 {
            let derived = phi_from_cpt_deg(layer.cpt_qc, overburden_pa.max(50_000.0));
            best = Some((derived, "cpt"));
        }
        if layer.spt_n > 0.0 {
            let derived = phi_from_spt_deg(layer.spt_n);
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
